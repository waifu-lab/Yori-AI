use std::{thread, time::Duration};
use tauri::{AppHandle, Emitter, Manager, Runtime};
use crate::backend::{BackendWs, types::{ClientMsg, UserEventMsg}};

const IDLE_TIMEOUT: Duration = Duration::from_secs(3 * 60);

#[cfg(target_os = "windows")]
fn cursor_pos() -> Option<(i32, i32)> {
    use windows::Win32::Foundation::POINT;
    use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;
    let mut p = POINT::default();
    unsafe { GetCursorPos(&mut p).ok().map(|_| (p.x, p.y)) }
}

#[cfg(target_os = "macos")]
mod macos_cursor {
    use std::ffi::c_void;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CGPoint {
        pub x: f64,
        pub y: f64,
    }

    #[link(name = "CoreGraphics", kind = "framework")]
    unsafe extern "C" {
        pub fn CGEventCreate(source: *mut c_void) -> *mut c_void;
        pub fn CGEventGetLocation(event: *mut c_void) -> CGPoint;
        pub fn CFRelease(cf: *const c_void);
    }
}

/// Returns cursor position in CoreGraphics logical points.
/// (0, 0) is the top-left of the main display, y increases downward —
/// same origin as Tauri's outer_position() before scale is applied.
#[cfg(target_os = "macos")]
fn cursor_pos_logical() -> Option<(f64, f64)> {
    use macos_cursor::*;
    use std::ptr;
    unsafe {
        let event = CGEventCreate(ptr::null_mut());
        if event.is_null() {
            return None;
        }
        let pt = CGEventGetLocation(event);
        CFRelease(event.cast());
        Some((pt.x, pt.y))
    }
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
                    if let Some(ws) = app.try_state::<BackendWs>() {
                        let _ = ws.send(ClientMsg::UserEvent(UserEventMsg { name: "idle".into(), data: serde_json::json!({}) }));
                    }
                } else if !should_be_idle && is_idle {
                    is_idle = false;
                    let _ = app.emit("user-active", ());
                    if let Some(ws) = app.try_state::<BackendWs>() {
                        let _ = ws.send(ClientMsg::UserEvent(UserEventMsg { name: "active".into(), data: serde_json::json!({}) }));
                    }
                }
            }

            let Some(win) = app.get_webview_window("live2d-preview") else {
                continue;
            };

            if !win.is_visible().unwrap_or(false) {
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

            // Obtain cursor position in the same physical-pixel coordinate space as
            // Tauri's outer_position(). On macOS, CGEventGetLocation returns logical
            // points; we convert using the window's DPI scale factor (avoids calling
            // the deprecated CGDisplayScaleFactor which was removed in macOS 26).
            #[cfg(target_os = "macos")]
            let cursor = {
                let scale = win.scale_factor().unwrap_or(1.0);
                cursor_pos_logical().map(|(lx, ly)| ((lx * scale) as i32, (ly * scale) as i32))
            };
            #[cfg(target_os = "windows")]
            let cursor = cursor_pos();
            #[cfg(not(any(target_os = "macos", target_os = "windows")))]
            let cursor: Option<(i32, i32)> = None;

            let Some((cx, cy)) = cursor else {
                continue;
            };

            let px = (cx - pos.x) as f64;
            let py = (cy - pos.y) as f64;

            let _ = win.emit("cursor-pos", serde_json::json!({ "x": px, "y": py }));
        }
    });
}
