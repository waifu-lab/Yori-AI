use cpal::traits::{DeviceTrait, HostTrait};
use serde_json::{json, Value};
use waifu_shared::NodeError;

pub async fn audio_devices(_params: &Value) -> Result<Value, NodeError> {
    tokio::task::spawn_blocking(list_devices)
        .await
        .map_err(|e| NodeError {
            code: "INVALID_ACTION".into(),
            message: e.to_string(),
        })?
}

fn list_devices() -> Result<Value, NodeError> {
    let host = cpal::default_host();

    let default_input = host
        .default_input_device()
        .and_then(|d| d.name().ok())
        .unwrap_or_default();

    let default_output = host
        .default_output_device()
        .and_then(|d| d.name().ok())
        .unwrap_or_default();

    let inputs: Vec<Value> = host
        .input_devices()
        .map_err(|e| NodeError {
            code: "INVALID_ACTION".into(),
            message: format!("failed to enumerate input devices: {e}"),
        })?
        .filter_map(|d| {
            let name = d.name().ok()?;
            Some(json!({
                "id": name.clone(),
                "name": name.clone(),
                "default": name == default_input
            }))
        })
        .collect();

    let outputs: Vec<Value> = host
        .output_devices()
        .map_err(|e| NodeError {
            code: "INVALID_ACTION".into(),
            message: format!("failed to enumerate output devices: {e}"),
        })?
        .filter_map(|d| {
            let name = d.name().ok()?;
            Some(json!({
                "id": name.clone(),
                "name": name.clone(),
                "default": name == default_output
            }))
        })
        .collect();

    Ok(json!({ "inputs": inputs, "outputs": outputs }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn audio_devices_returns_expected_shape_or_known_error() {
        match audio_devices(&json!({})).await {
            Ok(value) => {
                assert!(value.get("inputs").is_some());
                assert!(value.get("outputs").is_some());
            }
            Err(err) => {
                assert_eq!(err.code, "INVALID_ACTION");
            }
        }
    }
}
