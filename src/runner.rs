use anyhow::Result;
use std::process::Command;

pub struct TestResult {
    /// True when all scripts in the file passed (no failures, no compile errors).
    pub passed: bool,
    /// Raw combined output from daml test (stdout + stderr), for display.
    pub output: String,
    /// True when the failure is a compile error (vs a runtime assertion failure).
    pub compile_error: bool,
}

/// Run `daml test --files <file>` from the project root and return the result.
///
/// Output format emitted by daml test:
///   <file>:<script_name>: ok, N active contracts, N transactions.
///   <file>:<script_name>: failed
pub fn run(file: &str) -> Result<TestResult> {
    let output = Command::new("daml")
        .args(["test", "--files", file])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    let combined = format!("{stdout}{stderr}");

    // Non-zero exit code → compile error (daml test exits 1 on DsError diagnostics).
    let compile_error = !output.status.success();

    // Even with zero exit, individual scripts can fail at runtime.
    let has_failure = stdout.lines().any(|l| l.contains(": failed"));
    let has_ok = stdout.lines().any(|l| l.contains(": ok"));

    // A file with no scripts at all is not a pass.
    let passed = !compile_error && !has_failure && has_ok;

    Ok(TestResult {
        passed,
        output: combined,
        compile_error,
    })
}
