use base64::Engine;
use serde_json::{json, Value};
use waifu_shared::NodeError;
use xcap::Monitor;

pub async fn screenshot(params: &Value) -> Result<Value, NodeError> {
    let display_idx = parse_display(params)?;

    tokio::task::spawn_blocking(move || capture(display_idx))
        .await
        .map_err(|e| NodeError {
            code: "SCREENSHOT_FAILED".into(),
            message: e.to_string(),
        })?
}

fn parse_display(params: &Value) -> Result<i64, NodeError> {
    match params.get("display") {
        Some(value) => {
            let display_idx = value.as_i64().ok_or_else(|| NodeError {
                code: "INVALID_PARAMS".into(),
                message: "display must be an integer".into(),
            })?;

            if display_idx < -1 {
                return Err(NodeError {
                    code: "INVALID_PARAMS".into(),
                    message: "display must be -1 or greater".into(),
                });
            }

            Ok(display_idx)
        }
        None => Ok(0),
    }
}

fn capture(display_idx: i64) -> Result<Value, NodeError> {
    let monitors = Monitor::all().map_err(|e| NodeError {
        code: "SCREENSHOT_FAILED".into(),
        message: format!("failed to list monitors: {e}"),
    })?;

    if monitors.is_empty() {
        return Err(NodeError {
            code: "SCREENSHOT_FAILED".into(),
            message: "no monitors found".into(),
        });
    }

    let monitor = if display_idx == -1 {
        &monitors[0]
    } else {
        let idx = display_idx as usize;
        monitors.get(idx).ok_or_else(|| NodeError {
            code: "SCREENSHOT_FAILED".into(),
            message: format!(
                "display index {display_idx} out of range (0..{})",
                monitors.len()
            ),
        })?
    };

    let image = monitor.capture_image().map_err(|e| NodeError {
        code: "SCREENSHOT_FAILED".into(),
        message: format!("capture failed: {e}"),
    })?;

    let width = image.width();
    let height = image.height();

    let mut png_bytes: Vec<u8> = Vec::new();
    image
        .write_to(
            &mut std::io::Cursor::new(&mut png_bytes),
            xcap::image::ImageFormat::Png,
        )
        .map_err(|e| NodeError {
            code: "SCREENSHOT_FAILED".into(),
            message: format!("png encode failed: {e}"),
        })?;

    let b64 = base64::engine::general_purpose::STANDARD.encode(&png_bytes);

    Ok(json!({
        "image": b64,
        "mime_type": "image/png",
        "encoding": "base64",
        "width": width,
        "height": height,
        "display": display_idx
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn screenshot_rejects_non_integer_display() {
        let err = screenshot(&json!({ "display": "0" })).await.unwrap_err();
        assert_eq!(err.code, "INVALID_PARAMS");
    }

    #[tokio::test]
    async fn screenshot_rejects_invalid_negative_display() {
        let err = screenshot(&json!({ "display": -2 })).await.unwrap_err();
        assert_eq!(err.code, "INVALID_PARAMS");
    }

    #[tokio::test]
    async fn screenshot_invalid_display_returns_known_error_code() {
        let result = screenshot(&json!({ "display": 999_999 })).await;
        if let Err(err) = result {
            assert_eq!(err.code, "SCREENSHOT_FAILED");
        }
    }
}
