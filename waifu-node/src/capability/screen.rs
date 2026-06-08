use base64::Engine;
use serde_json::{json, Value};
use waifu_shared::NodeError;
use xcap::image::codecs::jpeg::JpegEncoder;
use xcap::image::{DynamicImage, ImageEncoder};
use xcap::Monitor;

const MAX_WIDTH: u32 = 800;
const MAX_HEIGHT: u32 = 450;
const JPEG_QUALITY: u8 = 70;

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

    let rgba = monitor.capture_image().map_err(|e| NodeError {
        code: "SCREENSHOT_FAILED".into(),
        message: format!("capture failed: {e}"),
    })?;

    let image = DynamicImage::ImageRgba8(rgba);

    // Downscale to cap — preserve aspect ratio.
    let image = if image.width() > MAX_WIDTH || image.height() > MAX_HEIGHT {
        image.resize(
            MAX_WIDTH,
            MAX_HEIGHT,
            xcap::image::imageops::FilterType::Lanczos3,
        )
    } else {
        image
    };

    // JPEG has no alpha; flatten to RGB first.
    let rgb = image.to_rgb8();
    let width = rgb.width();
    let height = rgb.height();

    let mut jpeg_bytes: Vec<u8> = Vec::new();
    let encoder = JpegEncoder::new_with_quality(&mut jpeg_bytes, JPEG_QUALITY);
    encoder
        .write_image(
            rgb.as_raw(),
            width,
            height,
            xcap::image::ExtendedColorType::Rgb8,
        )
        .map_err(|e| NodeError {
            code: "SCREENSHOT_FAILED".into(),
            message: format!("jpeg encode failed: {e}"),
        })?;

    let b64 = base64::engine::general_purpose::STANDARD.encode(&jpeg_bytes);

    Ok(json!({
        "__type": "image",
        "bytes": b64,
        "media_type": "image/jpeg",
        "description": format!("截圖 {}x{}", width, height)
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
