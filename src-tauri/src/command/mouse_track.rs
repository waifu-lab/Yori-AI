use std::{thread, time::Duration};
use tauri::{AppHandle, Emitter, Manager, Runtime};

#[cfg(target_os = "windows")]
fn cursor_pos() -> Option<(i32, i32)> {
    use windows::Win32::Foundation::POINT;
    use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;
    let mut p = POINT::default();
    unsafe { GetCursorPos(&mut p).ok().map(|_| (p.x, p.y)) }
}

#[cfg(not(target_os = "windows"))]
fn cursor_pos() -> Option<(i32, i32)> {
    None
}

/// Starts a background thread that emits global cursor position
/// (normalised to the live2d-preview window, -1..1) as "cursor-pos" events.
pub fn start_mouse_tracker<R: Runtime + 'static>(app: AppHandle<R>) {
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(16));

        let Some((cx, cy)) = cursor_pos() else {
            continue;
        };
        let Some(win) = app.get_webview_window("live2d-preview") else {
            continue;
        };

        // Skip if the window is hidden to avoid unnecessary events
        if win.is_visible().unwrap_or(false) == false {
            continue;
        }

        let Ok(pos) = win.outer_position() else {
            continue;
        };
        let Ok(size) = win.outer_size() else {
            continue;
        };
        if size.width == 0 || size.height == 0 {
            continue;
        }

        let px = (cx - pos.x) as f64;
        let py = (cy - pos.y) as f64;

        let _ = win.emit("cursor-pos", serde_json::json!({ "x": px, "y": py }));
    });
}
