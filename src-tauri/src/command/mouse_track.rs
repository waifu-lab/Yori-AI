use std::{thread, time::Duration};
use tauri::{AppHandle, Emitter, Manager, Runtime};

const IDLE_TIMEOUT: Duration = Duration::from_secs(3 * 60);

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

/// Returns how long ago the user last moved the mouse or pressed a key.
/// Uses GetLastInputInfo so both mouse and keyboard activity count.
#[cfg(target_os = "windows")]
fn last_input_elapsed() -> Option<Duration> {
    use windows::Win32::System::SystemInformation::GetTickCount64;
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};
    let mut lii = LASTINPUTINFO {
        cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
        dwTime: 0,
    };
    unsafe {
        if GetLastInputInfo(&mut lii).as_bool() {
            let now_ms = GetTickCount64();
            let elapsed_ms = now_ms.saturating_sub(lii.dwTime as u64);
            Some(Duration::from_millis(elapsed_ms))
        } else {
            None
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn last_input_elapsed() -> Option<Duration> {
    None
}

/// Starts a background thread that:
/// - Emits "cursor-pos" events (normalised to the live2d-preview window) at ~60 Hz.
/// - Emits "user-idle" once the user has had no mouse/keyboard input for IDLE_TIMEOUT.
/// - Emits "user-active" when the user returns after being idle.
pub fn start_mouse_tracker<R: Runtime + 'static>(app: AppHandle<R>) {
    thread::spawn(move || {
        let mut is_idle = false;

        loop {
            thread::sleep(Duration::from_millis(16));

            if let Some(elapsed) = last_input_elapsed() {
                let should_be_idle = elapsed >= IDLE_TIMEOUT;
                if should_be_idle && !is_idle {
                    is_idle = true;
                    let _ = app.emit("user-idle", ());
                } else if !should_be_idle && is_idle {
                    is_idle = false;
                    let _ = app.emit("user-active", ());
                }
            }

            let Some((cx, cy)) = cursor_pos() else {
                continue;
            };
            let Some(win) = app.get_webview_window("live2d-preview") else {
                continue;
            };

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
        }
    });
}
