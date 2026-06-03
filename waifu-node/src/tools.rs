use serde_json::json;
use waifu_shared::ToolSchema;

pub fn build_tool_schemas(caps: &[String]) -> Vec<ToolSchema> {
    let mut schemas = Vec::new();

    for cap in caps {
        match cap.as_str() {
            "screen" => schemas.extend(screen_tools()),
            "exec" => schemas.extend(exec_tools()),
            "sysinfo" => schemas.extend(sysinfo_tools()),
            "audio" => schemas.extend(audio_tools()),
            "input" => schemas.extend(input_tools()),
            _ => {}
        }
    }

    schemas
}

fn screen_tools() -> Vec<ToolSchema> {
    vec![
        ToolSchema {
            name: "screenshot".into(),
            description: "Capture a screenshot of a display and return a base64-encoded PNG."
                .into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "display": {
                        "type": "integer",
                        "description": "Display index to capture. 0 = first/primary monitor. Defaults to 0.",
                        "default": 0
                    }
                }
            }),
        },
        ToolSchema {
            name: "active_window".into(),
            description: "Get the currently focused window: title, process name, and process ID."
                .into(),
            parameters: json!({ "type": "object", "properties": {} }),
        },
        ToolSchema {
            name: "window_list".into(),
            description: "List all visible windows with their titles and process names.".into(),
            parameters: json!({ "type": "object", "properties": {} }),
        },
    ]
}

fn exec_tools() -> Vec<ToolSchema> {
    vec![ToolSchema {
        name: "exec".into(),
        description: "Execute a system command and return stdout, stderr, and exit code.".into(),
        parameters: json!({
            "type": "object",
            "properties": {
                "cmd":        { "type": "string",  "description": "Executable path or command name." },
                "args":       { "type": "array",   "items": { "type": "string" }, "default": [] },
                "timeout_ms": { "type": "integer", "description": "Timeout ms (0 = no limit).", "default": 10000 },
                "cwd":        { "type": "string",  "description": "Working directory (null = agent cwd)." }
            },
            "required": ["cmd"]
        }),
    }]
}

fn sysinfo_tools() -> Vec<ToolSchema> {
    vec![ToolSchema {
        name: "sysinfo".into(),
        description: "Get system info: CPU, memory, disk, network, OS, hostname.".into(),
        parameters: json!({ "type": "object", "properties": {} }),
    }]
}

fn audio_tools() -> Vec<ToolSchema> {
    vec![ToolSchema {
        name: "audio_devices".into(),
        description: "List audio input and output devices.".into(),
        parameters: json!({ "type": "object", "properties": {} }),
    }]
}

fn input_tools() -> Vec<ToolSchema> {
    vec![
        ToolSchema {
            name: "key_press".into(),
            description: "Press a key or key combination (e.g. ctrl+c, enter).".into(),
            parameters: json!({
                "type": "object",
                "properties": { "key": { "type": "string", "description": "Key or combination." } },
                "required": ["key"]
            }),
        },
        ToolSchema {
            name: "key_down".into(),
            description: "Hold a key down.".into(),
            parameters: json!({
                "type": "object",
                "properties": { "key": { "type": "string" } },
                "required": ["key"]
            }),
        },
        ToolSchema {
            name: "key_up".into(),
            description: "Release a held key.".into(),
            parameters: json!({
                "type": "object",
                "properties": { "key": { "type": "string" } },
                "required": ["key"]
            }),
        },
        ToolSchema {
            name: "mouse_move".into(),
            description: "Move the mouse cursor to an absolute screen position.".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "x": { "type": "integer" },
                    "y": { "type": "integer" }
                },
                "required": ["x", "y"]
            }),
        },
        ToolSchema {
            name: "mouse_click".into(),
            description: "Click the mouse at an absolute screen position.".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "x":      { "type": "integer" },
                    "y":      { "type": "integer" },
                    "button": { "type": "string", "enum": ["left", "right", "middle"], "default": "left" }
                },
                "required": ["x", "y"]
            }),
        },
        ToolSchema {
            name: "mouse_scroll".into(),
            description: "Scroll the mouse wheel at an absolute screen position.".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "x":  { "type": "integer" },
                    "y":  { "type": "integer" },
                    "dx": { "type": "integer", "default": 0 },
                    "dy": { "type": "integer", "description": "Negative = scroll down.", "default": -3 }
                },
                "required": ["x", "y"]
            }),
        },
        ToolSchema {
            name: "type_text".into(),
            description: "Type a string of text as if typed on a keyboard.".into(),
            parameters: json!({
                "type": "object",
                "properties": { "text": { "type": "string" } },
                "required": ["text"]
            }),
        },
    ]
}
