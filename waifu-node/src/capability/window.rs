use serde_json::{json, Value};
use waifu_shared::NodeError;

pub async fn active_window(_params: &Value) -> Result<Value, NodeError> {
    tokio::task::spawn_blocking(get_active_window)
        .await
        .map_err(|e| NodeError {
            code: "WINDOW_ERROR".into(),
            message: e.to_string(),
        })?
}

pub async fn window_list(_params: &Value) -> Result<Value, NodeError> {
    tokio::task::spawn_blocking(get_window_list)
        .await
        .map_err(|e| NodeError {
            code: "WINDOW_ERROR".into(),
            message: e.to_string(),
        })?
}

#[cfg(target_os = "windows")]
fn get_active_window() -> Result<Value, NodeError> {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};

    let hwnd: HWND = unsafe { GetForegroundWindow() };
    if hwnd.0 == 0 {
        return Ok(json!({ "title": "", "process_name": "", "process_id": 0 }));
    }

    let title = read_window_title(hwnd);
    let mut pid = 0u32;
    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };
    let process_name = process_name_from_pid(pid);

    Ok(json!({
        "title": title,
        "process_name": process_name,
        "process_id": pid
    }))
}

#[cfg(target_os = "windows")]
fn get_window_list() -> Result<Value, NodeError> {
    use std::sync::Mutex;
    use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
    use windows::Win32::UI::WindowsAndMessaging::{EnumWindows, IsWindowVisible};

    static RESULTS: Mutex<Vec<Value>> = Mutex::new(Vec::new());

    // Clear from any previous call
    if let Ok(mut g) = RESULTS.lock() {
        g.clear();
    }

    unsafe extern "system" fn callback(hwnd: HWND, _: LPARAM) -> BOOL {
        use windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;
        if unsafe { IsWindowVisible(hwnd) }.as_bool() {
            let title = read_window_title(hwnd);
            if !title.is_empty() {
                let mut pid = 0u32;
                unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid)) };
                let process_name = process_name_from_pid(pid);
                let entry =
                    json!({ "title": title, "process_name": process_name, "process_id": pid });
                if let Ok(mut g) = RESULTS.lock() {
                    g.push(entry);
                }
            }
        }
        BOOL(1)
    }

    let _ = unsafe { EnumWindows(Some(callback), LPARAM(0)) };

    let list = RESULTS.lock().map(|g| g.clone()).unwrap_or_default();
    Ok(Value::Array(list))
}

#[cfg(target_os = "windows")]
fn read_window_title(hwnd: windows::Win32::Foundation::HWND) -> String {
    use windows::Win32::UI::WindowsAndMessaging::GetWindowTextW;
    let mut buf = vec![0u16; 512];
    let len = unsafe { GetWindowTextW(hwnd, &mut buf) } as usize;
    String::from_utf16_lossy(&buf[..len])
}

#[cfg(target_os = "windows")]
fn process_name_from_pid(pid: u32) -> String {
    use sysinfo::{Pid, System};
    let mut sys = System::new();
    sys.refresh_process(Pid::from_u32(pid));
    sys.process(Pid::from_u32(pid))
        .map(|p| p.name().to_string())
        .unwrap_or_default()
}

#[cfg(not(target_os = "windows"))]
fn get_active_window() -> Result<Value, NodeError> {
    Err(NodeError {
        code: "NOT_SUPPORTED".into(),
        message: "active_window is only supported on Windows".into(),
    })
}

#[cfg(not(target_os = "windows"))]
fn get_window_list() -> Result<Value, NodeError> {
    Err(NodeError {
        code: "NOT_SUPPORTED".into(),
        message: "window_list is only supported on Windows".into(),
    })
}
