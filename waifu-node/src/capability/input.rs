use enigo::{
    Button, Coordinate,
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use serde_json::{json, Value};
use waifu_shared::NodeError;

fn make_enigo() -> Result<Enigo, NodeError> {
    Enigo::new(&Settings::default()).map_err(|e| NodeError {
        code: "INPUT_FAILED".into(),
        message: format!("failed to init enigo: {e}"),
    })
}

fn parse_key(s: &str) -> Result<Key, NodeError> {
    let lower = s.to_lowercase();
    let k = match lower.as_str() {
        "enter" | "return" => Key::Return,
        "tab" => Key::Tab,
        "space" => Key::Space,
        "backspace" => Key::Backspace,
        "delete" | "del" => Key::Delete,
        "escape" | "esc" => Key::Escape,
        "up" => Key::UpArrow,
        "down" => Key::DownArrow,
        "left" => Key::LeftArrow,
        "right" => Key::RightArrow,
        "home" => Key::Home,
        "end" => Key::End,
        "pageup" => Key::PageUp,
        "pagedown" => Key::PageDown,
        "f1" => Key::F1,
        "f2" => Key::F2,
        "f3" => Key::F3,
        "f4" => Key::F4,
        "f5" => Key::F5,
        "f6" => Key::F6,
        "f7" => Key::F7,
        "f8" => Key::F8,
        "f9" => Key::F9,
        "f10" => Key::F10,
        "f11" => Key::F11,
        "f12" => Key::F12,
        "ctrl" | "control" => Key::Control,
        "shift" => Key::Shift,
        "alt" => Key::Alt,
        "meta" | "win" | "super" => Key::Meta,
        "caps_lock" | "capslock" => Key::CapsLock,
        c if c.chars().count() == 1 => Key::Unicode(c.chars().next().unwrap()),
        other => {
            return Err(NodeError {
                code: "INVALID_PARAMS".into(),
                message: format!("unknown key: {other}"),
            })
        }
    };
    Ok(k)
}

fn require_str<'a>(params: &'a Value, field: &str) -> Result<&'a str, NodeError> {
    params
        .get(field)
        .and_then(|v| v.as_str())
        .ok_or_else(|| NodeError {
            code: "INVALID_PARAMS".into(),
            message: format!("missing required param: {field}"),
        })
}

fn require_i64(params: &Value, field: &str) -> Result<i64, NodeError> {
    params
        .get(field)
        .and_then(|v| v.as_i64())
        .ok_or_else(|| NodeError {
            code: "INVALID_PARAMS".into(),
            message: format!("missing required param: {field}"),
        })
}

pub async fn key_press(params: &Value) -> Result<Value, NodeError> {
    let key_str = require_str(params, "key")?.to_string();
    tokio::task::spawn_blocking(move || {
        // 處理組合鍵，如 "ctrl+c"
        let parts: Vec<&str> = key_str.split('+').collect();
        let mut enigo = make_enigo()?;

        if parts.len() > 1 {
            let modifiers = &parts[..parts.len() - 1];
            let main_key = parts[parts.len() - 1];
            for m in modifiers {
                let k = parse_key(m)?;
                enigo.key(k, Press).map_err(|e| NodeError {
                    code: "INPUT_FAILED".into(),
                    message: e.to_string(),
                })?;
            }
            let k = parse_key(main_key)?;
            enigo.key(k, Click).map_err(|e| NodeError {
                code: "INPUT_FAILED".into(),
                message: e.to_string(),
            })?;
            for m in modifiers.iter().rev() {
                let k = parse_key(m)?;
                enigo.key(k, Release).map_err(|e| NodeError {
                    code: "INPUT_FAILED".into(),
                    message: e.to_string(),
                })?;
            }
        } else {
            let k = parse_key(&key_str)?;
            enigo.key(k, Click).map_err(|e| NodeError {
                code: "INPUT_FAILED".into(),
                message: e.to_string(),
            })?;
        }

        Ok(json!({ "ok": true }))
    })
    .await
    .map_err(|e| NodeError {
        code: "INPUT_FAILED".into(),
        message: e.to_string(),
    })?
}

pub async fn key_down(params: &Value) -> Result<Value, NodeError> {
    let key_str = require_str(params, "key")?.to_string();
    tokio::task::spawn_blocking(move || {
        let k = parse_key(&key_str)?;
        let mut enigo = make_enigo()?;
        enigo.key(k, Press).map_err(|e| NodeError {
            code: "INPUT_FAILED".into(),
            message: e.to_string(),
        })?;
        Ok(json!({ "ok": true }))
    })
    .await
    .map_err(|e| NodeError {
        code: "INPUT_FAILED".into(),
        message: e.to_string(),
    })?
}

pub async fn key_up(params: &Value) -> Result<Value, NodeError> {
    let key_str = require_str(params, "key")?.to_string();
    tokio::task::spawn_blocking(move || {
        let k = parse_key(&key_str)?;
        let mut enigo = make_enigo()?;
        enigo.key(k, Release).map_err(|e| NodeError {
            code: "INPUT_FAILED".into(),
            message: e.to_string(),
        })?;
        Ok(json!({ "ok": true }))
    })
    .await
    .map_err(|e| NodeError {
        code: "INPUT_FAILED".into(),
        message: e.to_string(),
    })?
}

pub async fn mouse_move(params: &Value) -> Result<Value, NodeError> {
    let x = require_i64(params, "x")? as i32;
    let y = require_i64(params, "y")? as i32;
    tokio::task::spawn_blocking(move || {
        let mut enigo = make_enigo()?;
        enigo
            .move_mouse(x, y, Coordinate::Abs)
            .map_err(|e| NodeError {
                code: "INPUT_FAILED".into(),
                message: e.to_string(),
            })?;
        Ok(json!({ "ok": true }))
    })
    .await
    .map_err(|e| NodeError {
        code: "INPUT_FAILED".into(),
        message: e.to_string(),
    })?
}

pub async fn mouse_click(params: &Value) -> Result<Value, NodeError> {
    let x = require_i64(params, "x")? as i32;
    let y = require_i64(params, "y")? as i32;
    let button_str = params
        .get("button")
        .and_then(|v| v.as_str())
        .unwrap_or("left")
        .to_string();

    tokio::task::spawn_blocking(move || {
        let button = match button_str.as_str() {
            "left" => Button::Left,
            "right" => Button::Right,
            "middle" => Button::Middle,
            other => {
                return Err(NodeError {
                    code: "INVALID_PARAMS".into(),
                    message: format!("unknown button: {other}"),
                })
            }
        };
        let mut enigo = make_enigo()?;
        enigo
            .move_mouse(x, y, Coordinate::Abs)
            .map_err(|e| NodeError {
                code: "INPUT_FAILED".into(),
                message: e.to_string(),
            })?;
        enigo.button(button, Click).map_err(|e| NodeError {
            code: "INPUT_FAILED".into(),
            message: e.to_string(),
        })?;
        Ok(json!({ "ok": true }))
    })
    .await
    .map_err(|e| NodeError {
        code: "INPUT_FAILED".into(),
        message: e.to_string(),
    })?
}

pub async fn mouse_scroll(params: &Value) -> Result<Value, NodeError> {
    let x = require_i64(params, "x")? as i32;
    let y = require_i64(params, "y")? as i32;
    let dx = params.get("dx").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let dy = params.get("dy").and_then(|v| v.as_i64()).unwrap_or(-3) as i32;

    tokio::task::spawn_blocking(move || {
        let mut enigo = make_enigo()?;
        enigo
            .move_mouse(x, y, Coordinate::Abs)
            .map_err(|e| NodeError {
                code: "INPUT_FAILED".into(),
                message: e.to_string(),
            })?;
        if dx != 0 {
            enigo
                .scroll(dx, enigo::Axis::Horizontal)
                .map_err(|e| NodeError {
                    code: "INPUT_FAILED".into(),
                    message: e.to_string(),
                })?;
        }
        if dy != 0 {
            enigo
                .scroll(dy, enigo::Axis::Vertical)
                .map_err(|e| NodeError {
                    code: "INPUT_FAILED".into(),
                    message: e.to_string(),
                })?;
        }
        Ok(json!({ "ok": true }))
    })
    .await
    .map_err(|e| NodeError {
        code: "INPUT_FAILED".into(),
        message: e.to_string(),
    })?
}

pub async fn type_text(params: &Value) -> Result<Value, NodeError> {
    let text = require_str(params, "text")?.to_string();
    tokio::task::spawn_blocking(move || {
        let mut enigo = make_enigo()?;
        enigo.text(&text).map_err(|e| NodeError {
            code: "INPUT_FAILED".into(),
            message: e.to_string(),
        })?;
        Ok(json!({ "ok": true }))
    })
    .await
    .map_err(|e| NodeError {
        code: "INPUT_FAILED".into(),
        message: e.to_string(),
    })?
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn key_press_requires_key_param() {
        let err = key_press(&json!({}))
            .await
            .expect_err("missing key should fail");
        assert_eq!(err.code, "INVALID_PARAMS");
        assert!(err.message.contains("key"));
    }

    #[tokio::test]
    async fn mouse_move_requires_coordinates() {
        let err = mouse_move(&json!({ "x": 10 }))
            .await
            .expect_err("missing y should fail");
        assert_eq!(err.code, "INVALID_PARAMS");
        assert!(err.message.contains("y"));
    }

    #[tokio::test]
    async fn mouse_click_rejects_unknown_button() {
        let err = mouse_click(&json!({ "x": 0, "y": 0, "button": "side" }))
            .await
            .expect_err("unknown button should fail");
        assert_eq!(err.code, "INVALID_PARAMS");
        assert!(err.message.contains("unknown button"));
    }

    #[tokio::test]
    async fn type_text_requires_text_param() {
        let err = type_text(&json!({}))
            .await
            .expect_err("missing text should fail");
        assert_eq!(err.code, "INVALID_PARAMS");
        assert!(err.message.contains("text"));
    }
}
