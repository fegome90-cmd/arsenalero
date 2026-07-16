use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::{Child, ChildStderr, ChildStdin, ChildStdout, Command, ExitStatus, Stdio},
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    thread,
    time::{Duration, Instant},
};

use serde_json::{Value, json};

const PROTOCOL_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_RESPONSE_BYTES: usize = 1024 * 1024;
const TOOL_NAMES: [&str; 5] = [
    "arsenal_init",
    "arsenal_stage",
    "arsenal_issue",
    "arsenal_attest",
    "arsenal_reconcile",
];

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

struct StdioProcess {
    child: Child,
    stdin: Option<ChildStdin>,
    stdout: Option<BufReader<ChildStdout>>,
    stderr: Option<ChildStderr>,
    next_id: u64,
}

impl StdioProcess {
    fn spawn(allowed_root: Option<&Path>) -> Self {
        let executable = env!("CARGO_BIN_EXE_arsenalero-mcp");
        let mut command = Command::new(executable);
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if let Some(root) = allowed_root {
            let roots = std::env::join_paths([root.as_os_str()]).expect("allowed root path");
            command.env("ARSENALERO_ALLOWED_ROOTS", roots);
        }

        let mut child = command.spawn().expect("the stdio MCP process starts");
        Self {
            stdin: Some(child.stdin.take().expect("stdin is piped")),
            stdout: Some(BufReader::new(
                child.stdout.take().expect("stdout is piped"),
            )),
            stderr: Some(child.stderr.take().expect("stderr is piped")),
            child,
            next_id: 1,
        }
    }

    fn initialize(&mut self) -> Value {
        let response = self.request(
            "initialize",
            json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {"name": "task11-test", "version": "0.1.0"}
            }),
        );
        assert_eq!(response["id"], json!(1));
        assert_eq!(
            response["result"]["serverInfo"]["name"],
            json!("arsenalero")
        );
        assert_eq!(
            response["result"]["serverInfo"]["version"],
            json!(env!("CARGO_PKG_VERSION"))
        );
        self.notify("notifications/initialized", json!({}));
        response
    }

    fn request(&mut self, method: &str, params: Value) -> Value {
        let id = self.next_id;
        self.next_id += 1;
        self.write_json(&json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params,
        }));
        self.read_response(&format!("{method} response"))
    }

    fn raw_request(&mut self, raw: &str) -> Value {
        let stdin = self.stdin.as_mut().expect("stdin is available");
        stdin
            .write_all(raw.as_bytes())
            .expect("raw request is written");
        if !raw.ends_with('\n') {
            stdin.write_all(b"\n").expect("raw request is line framed");
        }
        stdin.flush().expect("raw request is flushed");
        self.read_response("raw protocol response")
    }

    fn notify(&mut self, method: &str, params: Value) {
        self.write_json(&json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
        }));
    }

    fn call_tool(&mut self, name: &str, arguments: Value) -> Value {
        self.request("tools/call", json!({"name": name, "arguments": arguments}))
    }

    fn write_json(&mut self, value: &Value) {
        let line = serde_json::to_string(value).expect("request serializes");
        let stdin = self.stdin.as_mut().expect("stdin is available");
        writeln!(stdin, "{line}").expect("request is written");
        stdin.flush().expect("request is flushed");
    }

    fn read_response(&mut self, response_name: &str) -> Value {
        let stdout = self.stdout.take().expect("stdout is available");
        let (stdout, response) = read_line_with_timeout(&mut self.child, stdout, response_name);
        self.stdout = Some(stdout);
        serde_json::from_str(&response).unwrap_or_else(|error| {
            panic!("{response_name} is not valid JSON: {error}; raw={response}")
        })
    }

    fn finish(mut self) {
        drop(self.stdin.take());
        let status = wait_for_exit(&mut self.child, "clean stdio termination");
        assert!(status.success(), "clean stdio termination: {status}");
        let mut stderr_output = String::new();
        self.stderr
            .take()
            .expect("stderr is available")
            .read_to_string(&mut stderr_output)
            .expect("stderr is readable after clean termination");
        assert!(
            stderr_output.trim().is_empty(),
            "unexpected server stderr: {stderr_output}"
        );
    }

    fn finish_after_protocol_error(mut self) -> ExitStatus {
        drop(self.stdin.take());
        let status = wait_for_exit(&mut self.child, "protocol-error stdio termination");
        let mut stderr_output = String::new();
        self.stderr
            .take()
            .expect("stderr is available")
            .read_to_string(&mut stderr_output)
            .expect("stderr is readable after protocol-error termination");
        status
    }
}

impl Drop for StdioProcess {
    fn drop(&mut self) {
        drop(self.stdin.take());
        if !matches!(self.child.try_wait(), Ok(Some(_))) {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures")
        .join(name)
}

static NEXT_TEMP_DIR: AtomicUsize = AtomicUsize::new(0);

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(label: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "arsenalero-task11-{label}-{}-{}",
            std::process::id(),
            NEXT_TEMP_DIR.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&path).expect("temporary fixture directory");
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn copy_fixture(name: &str) -> TempDir {
    let destination = TempDir::new(name);
    copy_tree(&fixture_path(name), destination.path());
    destination
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("fixture destination directory");
    for entry in fs::read_dir(source).expect("fixture directory is readable") {
        let entry = entry.expect("fixture directory entry");
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if entry.file_type().expect("fixture entry type").is_dir() {
            copy_tree(&source_path, &destination_path);
        } else {
            fs::copy(&source_path, &destination_path).expect("fixture file copy");
        }
    }
}

fn successful_tool_output(response: &Value) -> Value {
    assert!(
        response.get("error").is_none(),
        "unexpected JSON-RPC error: {response}"
    );
    assert_eq!(response["result"]["isError"], json!(false));
    response["result"]["structuredContent"].clone()
}

fn assert_tool_error(response: &Value, expected_code: &str) {
    assert!(
        response.get("error").is_none(),
        "domain error became protocol error: {response}"
    );
    assert_eq!(response["result"]["isError"], json!(true));
    assert_eq!(
        response["result"]["structuredContent"]["error_code"],
        json!(expected_code)
    );
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .expect("tool error has JSON text content");
    assert_eq!(
        serde_json::from_str::<Value>(text).expect("tool error text is JSON"),
        response["result"]["structuredContent"]
    );
}

fn init_case(client: &mut StdioProcess, root: &Path) -> Value {
    let output = successful_tool_output(&client.call_tool(
        "arsenal_init",
        json!({
            "skill_root": root.to_string_lossy(),
            "task_summary": "verify Task 11 stdio custody",
            "operation": "implementation"
        }),
    ));
    assert_eq!(output["status"], json!("ready"));
    output
}

#[test]
fn stdio_bootstrap_negotiates_the_exact_five_tool_mcp_surface() {
    let mut server = StdioProcess::spawn(None);
    let initialize_response = server.initialize();
    assert_eq!(
        initialize_response["result"]["capabilities"]["tools"],
        json!({})
    );
    assert!(initialize_response["result"].get("resources").is_none());
    assert!(initialize_response["result"].get("prompts").is_none());

    let tools_response = server.request("tools/list", json!({}));
    let tools = tools_response["result"]["tools"]
        .as_array()
        .expect("tools/list returns a tool array");
    let names = tools
        .iter()
        .map(|tool| tool["name"].as_str().expect("tool name"))
        .collect::<Vec<_>>();
    assert_eq!(names, TOOL_NAMES);
    assert!(
        tools
            .iter()
            .all(|tool| tool.get("inputSchema").is_some() && tool.get("outputSchema").is_some())
    );
    server.finish();
}

#[test]
fn stdio_complete_case_reaches_complete_reconciliation() {
    let root = fixture_path("complete_case");
    let mut server = StdioProcess::spawn(Some(&root));
    server.initialize();

    let init = init_case(&mut server, &root);
    assert_eq!(init["required_resource_ids"], json!(["resources::guide"]));

    let case_id = init["case_id"].as_str().expect("case id").to_owned();
    let stage = successful_tool_output(&server.call_tool(
        "arsenal_stage",
        json!({
            "case_id": case_id,
            "stage": "implementation",
            "current_intent": "use the required guide"
        }),
    ));
    assert_eq!(stage["required_now"], json!(["resources::guide"]));

    let issue = successful_tool_output(&server.call_tool(
        "arsenal_issue",
        json!({"case_id": case_id, "resource_ids": ["resources::guide"]}),
    ));
    let receipt = issue["resources"][0]["receipt_id"]
        .as_str()
        .expect("receipt id")
        .to_owned();
    assert_eq!(
        issue["resources"][0]["resource_id"],
        json!("resources::guide")
    );

    let attest = successful_tool_output(&server.call_tool(
        "arsenal_attest",
        json!({
            "case_id": case_id,
            "attestations": [{"receipt_id": receipt, "usage": "used the guide"}]
        }),
    ));
    assert_eq!(
        attest["attestations"][0]["attained_evidence_level"],
        json!("attestation")
    );

    let reconcile =
        successful_tool_output(&server.call_tool("arsenal_reconcile", json!({"case_id": case_id})));
    assert_eq!(reconcile["status"], json!("complete"));
    assert_eq!(reconcile["protocol_completion"]["required"], json!(1));
    assert_eq!(reconcile["protocol_completion"]["issued"], json!(1));
    assert_eq!(reconcile["protocol_completion"]["attested"], json!(1));
    assert_eq!(reconcile["missing_attestations"], json!([]));
    server.finish();
}

#[test]
fn stdio_incomplete_case_reports_missing_issue_and_attestation() {
    let root = fixture_path("complete_case");
    let mut server = StdioProcess::spawn(Some(&root));
    server.initialize();
    let init = init_case(&mut server, &root);
    let case_id = init["case_id"].as_str().expect("case id");

    let reconcile =
        successful_tool_output(&server.call_tool("arsenal_reconcile", json!({"case_id": case_id})));
    assert_eq!(reconcile["status"], json!("incomplete"));
    assert_eq!(reconcile["protocol_completion"]["issued"], json!(0));
    assert_eq!(reconcile["protocol_completion"]["attested"], json!(0));
    assert_eq!(
        reconcile["required_but_never_issued"],
        json!(["resources::guide"])
    );
    server.finish();
}

#[test]
fn malformed_stdio_request_returns_json_rpc_protocol_error() {
    let mut server = StdioProcess::spawn(None);
    let response = server.raw_request(r#"{"jsonrpc":"2.0","id":7,"method":42,"params":{}}"#);
    assert_eq!(response["id"], Value::Null);
    assert_eq!(response["error"]["code"], json!(-32600));
    assert!(response["error"]["message"].is_string());
    assert!(
        !server.finish_after_protocol_error().success(),
        "malformed request must terminate the server with an error status"
    );
}

#[test]
fn domain_failure_is_a_tool_execution_error_not_a_protocol_error() {
    let mut server = StdioProcess::spawn(None);
    server.initialize();
    let response = server.call_tool(
        "arsenal_stage",
        json!({
            "case_id": "00000000-0000-7000-8000-000000000000",
            "stage": "verification",
            "current_intent": "run checks"
        }),
    );
    assert_tool_error(&response, "CASE_UNKNOWN");
    server.finish();
}

#[test]
fn path_escape_fixture_is_rejected_over_stdio() {
    let root = fixture_path("path_escape");
    let escaped_root = root.join("..");
    let mut server = StdioProcess::spawn(Some(&root));
    server.initialize();
    let response = server.call_tool(
        "arsenal_init",
        json!({
            "skill_root": escaped_root.to_string_lossy(),
            "task_summary": "path escape",
            "operation": "verification"
        }),
    );
    assert_tool_error(&response, "SKILL_ROOT_NOT_ALLOWED");
    server.finish();
}

#[cfg(unix)]
#[test]
fn symlink_escape_fixture_is_rejected_over_stdio() {
    use std::os::unix::fs::symlink;

    let root = copy_fixture("symlink_escape");
    fs::create_dir_all(root.path().join("resources")).expect("resource directory");
    let outside = TempDir::new("symlink-outside");
    fs::write(outside.path().join("outside.md"), "outside").expect("outside resource");
    symlink(
        outside.path().join("outside.md"),
        root.path().join("resources/escape.md"),
    )
    .expect("escaping fixture symlink");

    let mut server = StdioProcess::spawn(Some(root.path()));
    server.initialize();
    let response = server.call_tool(
        "arsenal_init",
        json!({
            "skill_root": root.path().to_string_lossy(),
            "task_summary": "symlink escape",
            "operation": "verification"
        }),
    );
    assert_tool_error(&response, "RESOURCE_SYMLINK_ESCAPE");
    server.finish();
}

#[test]
fn broken_reference_fixture_is_rejected_over_stdio() {
    let root = fixture_path("broken_reference");
    let mut server = StdioProcess::spawn(Some(&root));
    server.initialize();
    let response = server.call_tool(
        "arsenal_init",
        json!({
            "skill_root": root.to_string_lossy(),
            "task_summary": "broken reference",
            "operation": "verification"
        }),
    );
    assert_tool_error(&response, "RESOURCE_REFERENCE_BROKEN");
    server.finish();
}

#[test]
fn oversized_resource_fixture_is_rejected_over_stdio() {
    let root = copy_fixture("oversized_resource");
    fs::create_dir_all(root.path().join("resources")).expect("resource directory");
    fs::write(
        root.path().join("resources/oversized.md"),
        "x".repeat(256 * 1024 + 1),
    )
    .expect("oversized resource");

    let mut server = StdioProcess::spawn(Some(root.path()));
    server.initialize();
    let response = server.call_tool(
        "arsenal_init",
        json!({
            "skill_root": root.path().to_string_lossy(),
            "task_summary": "oversized resource",
            "operation": "verification"
        }),
    );
    assert_tool_error(&response, "RESOURCE_TOO_LARGE");
    server.finish();
}

#[test]
fn duplicate_id_fixture_is_rejected_over_stdio() {
    let root = fixture_path("duplicate_id");
    let mut server = StdioProcess::spawn(Some(&root));
    server.initialize();
    let response = server.call_tool(
        "arsenal_init",
        json!({
            "skill_root": root.to_string_lossy(),
            "task_summary": "duplicate resource id",
            "operation": "verification"
        }),
    );
    assert_tool_error(&response, "RESOURCE_ID_COLLISION");
    server.finish();
}

#[test]
fn cross_case_receipt_fixture_is_rejected_over_stdio() {
    let root = fixture_path("complete_case");
    let mut server = StdioProcess::spawn(Some(&root));
    server.initialize();
    let first = init_case(&mut server, &root);
    let second = init_case(&mut server, &root);
    let first_case = first["case_id"].as_str().expect("first case id");
    let second_case = second["case_id"].as_str().expect("second case id");

    let issue = successful_tool_output(&server.call_tool(
        "arsenal_issue",
        json!({"case_id": first_case, "resource_ids": ["resources::guide"]}),
    ));
    let receipt = issue["resources"][0]["receipt_id"]
        .as_str()
        .expect("receipt id")
        .to_owned();
    let response = server.call_tool(
        "arsenal_attest",
        json!({
            "case_id": second_case,
            "attestations": [{"receipt_id": receipt, "usage": "cross-case use"}]
        }),
    );
    assert_tool_error(&response, "RECEIPT_UNKNOWN");
    server.finish();
}

#[test]
fn resource_drift_fixture_is_reported_as_stale_over_stdio() {
    let root = copy_fixture("resource_drift");
    let mut server = StdioProcess::spawn(Some(root.path()));
    server.initialize();
    let init = init_case(&mut server, root.path());
    let case_id = init["case_id"].as_str().expect("case id").to_owned();
    let resource_id = init["required_resource_ids"][0]
        .as_str()
        .expect("required resource id")
        .to_owned();
    let _issue = successful_tool_output(&server.call_tool(
        "arsenal_issue",
        json!({"case_id": case_id, "resource_ids": [resource_id]}),
    ));
    fs::write(
        root.path().join("resources/guide.md"),
        "changed after issue\n",
    )
    .expect("resource drift");

    let reconcile =
        successful_tool_output(&server.call_tool("arsenal_reconcile", json!({"case_id": case_id})));
    assert_eq!(reconcile["status"], json!("needs_review"));
    assert_eq!(reconcile["stale_receipts"], json!(["resources::guide"]));
    server.finish();
}

#[test]
fn skill_drift_fixture_invalidates_the_case_over_stdio() {
    let root = copy_fixture("skill_drift");
    let mut server = StdioProcess::spawn(Some(root.path()));
    server.initialize();
    let init = init_case(&mut server, root.path());
    let case_id = init["case_id"].as_str().expect("case id").to_owned();
    let _issue = successful_tool_output(&server.call_tool(
        "arsenal_issue",
        json!({"case_id": case_id, "resource_ids": ["resources::guide"]}),
    ));
    fs::write(
        root.path().join("SKILL.md"),
        "# Skill drift\n\nThe skill changed after initialization.\n",
    )
    .expect("skill drift");

    let reconcile =
        successful_tool_output(&server.call_tool("arsenal_reconcile", json!({"case_id": case_id})));
    assert_eq!(reconcile["status"], json!("invalidated"));
    assert_eq!(reconcile["stale_receipts"], json!([]));
    server.finish();
}
