use serde_json::Value;
use waifu_shared::{NodeError, NodeInvoke, NodeResult};

use crate::capability;

pub async fn handle_invoke(invoke: &NodeInvoke) -> NodeResult {
    let result = dispatch_action(&invoke.action, &invoke.params).await;

    match result {
        Ok(data) => NodeResult {
            msg_type: "node_result".into(),
            request_id: invoke.request_id,
            ok: true,
            data: Some(data),
            error: None,
        },
        Err(e) => NodeResult {
            msg_type: "node_result".into(),
            request_id: invoke.request_id,
            ok: false,
            data: None,
            error: Some(e),
        },
    }
}

async fn dispatch_action(action: &str, params: &Value) -> Result<Value, NodeError> {
    match action {
        // screen
        "screenshot" => capability::screen::screenshot(params).await,

        // window
        "active_window" => capability::window::active_window(params).await,
        "window_list" => capability::window::window_list(params).await,

        // exec
        "exec" => capability::exec::exec(params).await,

        // sysinfo
        "sysinfo" => capability::sysinfo::sysinfo(params).await,

        // audio
        "audio_devices" => capability::audio::audio_devices(params).await,

        // input
        "key_press" => capability::input::key_press(params).await,
        "key_down" => capability::input::key_down(params).await,
        "key_up" => capability::input::key_up(params).await,
        "mouse_move" => capability::input::mouse_move(params).await,
        "mouse_click" => capability::input::mouse_click(params).await,
        "mouse_scroll" => capability::input::mouse_scroll(params).await,
        "type_text" => capability::input::type_text(params).await,

        other => Err(NodeError {
            code: "INVALID_ACTION".into(),
            message: format!("unknown action: {other}"),
        }),
    }
}
