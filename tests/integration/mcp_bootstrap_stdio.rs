use std::{
    io::{BufRead, BufReader, Read, Write},
    process::{Child, ChildStdout, Command, ExitStatus, Stdio},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

const PROTOCOL_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_RESPONSE_BYTES: usize = 1024 * 1024;

fn read_bounded_response(stdout: &mut BufReader<ChildStdout>) -> std::io::Result<String> {
    let mut response = Vec::new();

    loop {
        let buffer = stdout.fill_buf()?;
        if buffer.is_empty() {
            break;
        }

        let bytes_to_read = buffer
            .iter()
            .position(|byte| *byte == b'\n')
            .map_or(buffer.len(), |newline| newline + 1);

        if response.len() + bytes_to_read > MAX_RESPONSE_BYTES {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("MCP response exceeds {MAX_RESPONSE_BYTES} byte limit"),
            ));
        }

        response.extend_from_slice(&buffer[..bytes_to_read]);
        stdout.consume(bytes_to_read);

        if response.ends_with(b"\n") {
            break;
        }
    }

    String::from_utf8(response)
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))
}

fn read_line_with_timeout(
    child: &mut Child,
    stdout: BufReader<ChildStdout>,
    response_name: &str,
) -> (BufReader<ChildStdout>, String) {
    let (sender, receiver) = mpsc::sync_channel(1);

    thread::spawn(move || {
        let mut stdout = stdout;
        let result = read_bounded_response(&mut stdout);
        let _ = sender.send((stdout, result));
    });

    match receiver.recv_timeout(PROTOCOL_TIMEOUT) {
        Ok((stdout, Ok(response))) => (stdout, response),
        Ok((_, Err(error))) => panic!("{response_name} is unreadable: {error}"),
        Err(mpsc::RecvTimeoutError::Timeout) => {
            terminate_after_timeout(child, response_name);
            panic!("timed out waiting for {response_name} after {PROTOCOL_TIMEOUT:?}");
        }
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            panic!("{response_name} reader stopped before returning a response");
        }
    }
}

fn wait_for_exit(child: &mut Child, termination_name: &str) -> ExitStatus {
    let deadline = Instant::now() + PROTOCOL_TIMEOUT;

    loop {
        match child.try_wait() {
            Ok(Some(status)) => return status,
            Ok(None) if Instant::now() < deadline => thread::sleep(Duration::from_millis(10)),
            Ok(None) => {
                terminate_after_timeout(child, termination_name);
                panic!("timed out waiting for {termination_name} after {PROTOCOL_TIMEOUT:?}");
            }
            Err(error) => panic!("cannot observe {termination_name}: {error}"),
        }
    }
}

fn terminate_after_timeout(child: &mut Child, operation: &str) {
    if let Ok(Some(_)) = child.try_wait() {
        return;
    }

    child.kill().unwrap_or_else(|error| {
        panic!("cannot terminate MCP process after {operation} timed out: {error}")
    });

    let cleanup_deadline = Instant::now() + PROTOCOL_TIMEOUT;
    while Instant::now() < cleanup_deadline {
        match child.try_wait() {
            Ok(Some(_)) => return,
            Ok(None) => thread::sleep(Duration::from_millis(10)),
            Err(error) => {
                panic!("cannot observe MCP process cleanup after {operation} timed out: {error}")
            }
        }
    }

    panic!("MCP process did not terminate after {operation} timed out");
}

#[test]
fn stdio_bootstrap_negotiates_the_five_tool_mcp_surface() {
    let executable = env!("CARGO_BIN_EXE_arsenalero-mcp");
    let mut child = Command::new(executable)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the stdio MCP process starts");
    let mut stdin = child.stdin.take().expect("stdin is piped");
    let stdout = child.stdout.take().expect("stdout is piped");
    let mut stderr = child.stderr.take().expect("stderr is piped");
    let stdout = BufReader::new(stdout);

    writeln!(
        stdin,
        "{{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{{\"protocolVersion\":\"2024-11-05\",\"capabilities\":{{}},\"clientInfo\":{{\"name\":\"bootstrap-test\",\"version\":\"0.1.0\"}}}}}}"
    )
    .expect("initialize request is written");
    stdin.flush().expect("initialize request is flushed");

    let (stdout, initialize_response) =
        read_line_with_timeout(&mut child, stdout, "initialize response");
    assert!(initialize_response.contains("\"serverInfo\":{\"name\":\"arsenalero\""));
    assert!(
        initialize_response.contains(&format!("\"version\":\"{}\"", env!("CARGO_PKG_VERSION")))
    );
    assert!(initialize_response.contains("\"tools\":{}"));
    assert!(!initialize_response.contains("\"resources\":"));
    assert!(!initialize_response.contains("\"prompts\":"));

    writeln!(
        stdin,
        "{{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}}"
    )
    .expect("initialized notification is written");
    writeln!(
        stdin,
        "{{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/list\",\"params\":{{}}}}"
    )
    .expect("tools/list request is written");
    stdin.flush().expect("tools/list request is flushed");

    let (_stdout, tools_response) =
        read_line_with_timeout(&mut child, stdout, "tools/list response");
    assert!(tools_response.contains("\"id\":2"));
    for tool in [
        "arsenal_init",
        "arsenal_stage",
        "arsenal_issue",
        "arsenal_attest",
        "arsenal_reconcile",
    ] {
        assert!(
            tools_response.contains(tool),
            "missing tool {tool}: {tools_response}"
        );
    }
    assert_eq!(tools_response.matches("\"name\":\"arsenal_").count(), 5);

    drop(stdin);
    assert!(
        wait_for_exit(&mut child, "clean stdio termination").success(),
        "clean stdio termination"
    );
    let mut stderr_output = String::new();
    stderr
        .read_to_string(&mut stderr_output)
        .expect("stderr is readable after clean termination");
    assert!(
        stderr_output.trim().is_empty(),
        "unexpected stderr: {stderr_output}"
    );
}
