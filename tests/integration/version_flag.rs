//! Version-flag and no-flag launch tests for the `arsenalero-mcp` binary.
//!
//! ── TWO DIFFERENT NAMES — DO NOT CONFLATE ──────────────────────────
//!
//! 1. BINARY NAME / Cargo package name: `arsenalero-mcp`.
//!    Cargo exposes the built executable through the env var
//!    `CARGO_BIN_EXE_arsenalero-mcp` (WITH the `-mcp` suffix). Every
//!    `Command::new(...)` spawn below MUST use this form to locate the
//!    binary on disk. The package name in
//!    `crates/arsenalero-mcp/Cargo.toml` is `arsenalero-mcp`, and there
//!    is no `[[bin]]` override, so the binary name equals the package
//!    name. Using `env!("CARGO_BIN_EXE_arsenalero")` (no `-mcp`) is a
//!    BUG: that env var does not exist and compilation fails.
//!
//! 2. VERSION-STRING NAME: `arsenalero` (NO `-mcp` suffix).
//!    The binary's `--version` output must print `arsenalero <version>`
//!    to match the MCP `serverInfo.name` declared in `server.rs`:
//!        Implementation::new("arsenalero", env!("CARGO_PKG_VERSION"))
//!
//! So: spawn WITH `-mcp`  -> `env!("CARGO_BIN_EXE_arsenalero-mcp")`
//!     assert the line    -> `concat!("arsenalero ", env!("CARGO_PKG_VERSION"), "\n")`
//!     (WITHOUT `-mcp`, per V3).
//!
//! ## Coverage note (M4)
//!
//! The `no_flags_keeps_the_stdio_server_alive` test verifies ONLY that the
//! binary did not take the early-exit `--version` branch. It does NOT verify
//! that the MCP loop is actively serving requests. Full MCP loop coverage
//! (initialize handshake, tools/list, etc.) is provided by the existing
//! `tests/integration/mcp_bootstrap_stdio.rs`. The still-alive assertion
//! catches: (a) the version flag being mis-wired to short-circuit on no
//! args, (b) the binary crashing during startup. It does NOT catch: tokio
//! runtime initialization deadlocks (theoretical), rmcp transport binding
//! failures that don't cause exit. These are accepted limitations; deeper
//! coverage belongs in mcp_bootstrap_stdio.rs.

use std::io::Read;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

/// Hard ceiling for any single launch probe. Version flags must return
/// within this window; the no-flags server must still be alive at the
/// end of it.
const VERSION_TIMEOUT: Duration = Duration::from_secs(3);

/// Expected `--version` / `-V` output. The product name is the MCP
/// `serverInfo.name` (`arsenalero`, NO `-mcp`), and the version is the
/// workspace `CARGO_PKG_VERSION`. A trailing newline is included
/// because the binary prints the line with `println!`.
const EXPECTED_VERSION_LINE: &str = concat!("arsenalero ", env!("CARGO_PKG_VERSION"), "\n");

/// RAII guard that kills and reaps a child on drop.
///
/// Used ONLY for the no-flags test, whose process blocks on stdin and
/// would otherwise outlive the test. Tests that exit on their own
/// (`--version`, `-V`) do not need it.
struct ChildGuard(Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

/// Spawns `arsenalero-mcp <flag>`, waits for it to exit within
/// `VERSION_TIMEOUT`, and returns its stdout. Panics on timeout,
/// non-zero exit, or unreadable output.
fn run_version_command(flag: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_arsenalero-mcp"))
        .arg(flag)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|error| panic!("cannot spawn arsenalero-mcp {flag}: {error}"));

    let deadline = Instant::now() + VERSION_TIMEOUT;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                assert!(status.success(), "`{flag}` exit status: {status}");
                let mut stdout = String::new();
                child
                    .stdout
                    .take()
                    .expect("piped stdout")
                    .read_to_string(&mut stdout)
                    .expect("stdout is readable");
                return stdout;
            }
            Ok(None) if Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(20));
            }
            Ok(None) => {
                let _ = child.kill();
                let _ = child.wait();
                panic!("`{flag}` did not exit within {VERSION_TIMEOUT:?}");
            }
            Err(error) => panic!("cannot observe `{flag}` process: {error}"),
        }
    }
}

#[test]
fn long_version_flag_prints_package_version_and_exits_zero() {
    let stdout = run_version_command("--version");
    assert_eq!(
        stdout, EXPECTED_VERSION_LINE,
        "`--version` must print the MCP serverInfo name (`arsenalero`, no `-mcp`) followed by the version"
    );
}

#[test]
fn short_version_flag_prints_package_version_and_exits_zero() {
    let stdout = run_version_command("-V");
    assert_eq!(
        stdout, EXPECTED_VERSION_LINE,
        "`-V` must print the same version line as `--version`"
    );
}

#[test]
fn no_flags_keeps_the_stdio_server_alive() {
    // stdin is piped (and never written) so the stdio server blocks
    // reading input instead of receiving EOF and exiting early.
    let mut child = ChildGuard(
        Command::new(env!("CARGO_BIN_EXE_arsenalero-mcp"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap_or_else(|error| panic!("cannot spawn arsenalero-mcp with no flags: {error}")),
    );

    let deadline = Instant::now() + VERSION_TIMEOUT;
    while Instant::now() < deadline {
        match child.0.try_wait() {
            Ok(Some(status)) => {
                panic!("no-flags stdio server exited early without input: {status}");
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(20)),
            Err(error) => panic!("cannot observe no-flags process: {error}"),
        }
    }
    // Still alive after the timeout window: the stdio server is
    // correctly blocked waiting for input. ChildGuard kills and reaps
    // it on drop.
}
