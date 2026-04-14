use serde_json::{json, Value};
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;
use waifu_shared::NodeError;

pub async fn exec(params: &Value) -> Result<Value, NodeError> {
    let cmd = params
        .get("cmd")
        .and_then(|v| v.as_str())
        .ok_or_else(|| NodeError {
            code: "INVALID_PARAMS".into(),
            message: "missing required param: cmd".into(),
        })?
        .to_string();

    let args: Vec<String> = params
        .get("args")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let timeout_ms = params
        .get("timeout_ms")
        .and_then(|v| v.as_u64())
        .unwrap_or(10_000);

    let cwd = params
        .get("cwd")
        .and_then(|v| v.as_str())
        .map(String::from);

    run_command(cmd, args, timeout_ms, cwd).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn exec_requires_cmd_param() {
        let err = exec(&json!({})).await.expect_err("missing cmd should fail");
        assert_eq!(err.code, "INVALID_PARAMS");
        assert!(err.message.contains("cmd"));
    }

    #[tokio::test]
    async fn exec_invalid_command_returns_exec_failed() {
        let err = exec(&json!({ "cmd": "__definitely_missing_command__" }))
            .await
            .expect_err("invalid command should fail to start");
        assert_eq!(err.code, "EXEC_FAILED");
    }
}

async fn run_command(
    cmd: String,
    args: Vec<String>,
    timeout_ms: u64,
    cwd: Option<String>,
) -> Result<Value, NodeError> {
    let mut command = Command::new(&cmd);
    command.args(&args);

    if let Some(dir) = cwd {
        command.current_dir(dir);
    }

    let fut = command.output();

    let output = if timeout_ms == 0 {
        fut.await.map_err(|e| NodeError {
            code: "EXEC_FAILED".into(),
            message: format!("failed to start process: {e}"),
        })?
    } else {
        match timeout(Duration::from_millis(timeout_ms), fut).await {
            Ok(Ok(out)) => out,
            Ok(Err(e)) => {
                return Err(NodeError {
                    code: "EXEC_FAILED".into(),
                    message: format!("failed to start process: {e}"),
                })
            }
            Err(_) => {
                return Ok(json!({
                    "exit_code": null,
                    "stdout": "",
                    "stderr": "",
                    "timed_out": true
                }))
            }
        }
    };

    Ok(json!({
        "exit_code": output.status.code(),
        "stdout": String::from_utf8_lossy(&output.stdout),
        "stderr": String::from_utf8_lossy(&output.stderr),
        "timed_out": false
    }))
}
