use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, Runtime, State};

const INTERVAL_MIN_SECS: u64 = 5 * 60;
const INTERVAL_MAX_SECS: u64 = 10 * 60;
const SPEED_PX_PER_SEC: f64 = 120.0;
const STEP_MS: u64 = 16;

fn rand_next(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

/// Random integer in [lo, hi).
fn rand_range(s: &mut u64, lo: i32, hi: i32) -> i32 {
    if hi <= lo {
        return lo;
    }
    lo + (rand_next(s) % (hi - lo) as u64) as i32
}

pub struct WanderState {
    pub enabled: Arc<AtomicBool>,
    /// Set to true by `wander_now` to skip the current sleep.
    pub trigger_now: Arc<AtomicBool>,
}

impl WanderState {
    pub fn new() -> Self {
        Self {
            enabled: Arc::new(AtomicBool::new(true)),
            trigger_now: Arc::new(AtomicBool::new(false)),
        }
    }
}

fn smooth_move_to<R: Runtime>(
    win: &tauri::WebviewWindow<R>,
    target_x: i32,
    target_y: i32,
    enabled: &AtomicBool,
) {
    let px_per_step = SPEED_PX_PER_SEC * STEP_MS as f64 / 1000.0;

    loop {
        if !enabled.load(Ordering::Relaxed) {
            break;
        }
        let Ok(pos) = win.outer_position() else {
            break;
        };

        let dx = (target_x - pos.x) as f64;
        let dy = (target_y - pos.y) as f64;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist < 2.0 {
            let _ = win.set_position(PhysicalPosition::new(target_x, target_y));
            break;
        }

        let step = px_per_step.min(dist);
        let nx = pos.x + ((dx / dist) * step) as i32;
        let ny = pos.y + ((dy / dist) * step) as i32;
        let _ = win.set_position(PhysicalPosition::new(nx, ny));

        thread::sleep(Duration::from_millis(STEP_MS));
    }
}

pub fn start_wander_loop<R: Runtime + 'static>(
    app: AppHandle<R>,
    enabled: Arc<AtomicBool>,
    trigger_now: Arc<AtomicBool>,
) {
    thread::spawn(move || {
        let mut rng: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0xdeadbeef);

        loop {
            // Wait a random interval, waking every second.
            // The wait is skipped immediately if trigger_now is set.
            let wait = rand_range(&mut rng, INTERVAL_MIN_SECS as i32, INTERVAL_MAX_SECS as i32);
            for _ in 0..wait {
                if trigger_now.load(Ordering::Relaxed) {
                    break;
                }
                thread::sleep(Duration::from_secs(1));
            }
            trigger_now.store(false, Ordering::Relaxed);

            if !enabled.load(Ordering::Relaxed) {
                continue;
            }

            let Some(win) = app.get_webview_window("live2d-preview") else {
                continue;
            };
            if !win.is_visible().unwrap_or(false) {
                continue;
            }

            // Compute a random destination that fits within the current monitor.
            let Ok(win_size) = win.outer_size() else {
                continue;
            };
            let Ok(Some(monitor)) = win.current_monitor() else {
                continue;
            };

            let mx = monitor.position().x;
            let my = monitor.position().y;
            let max_x = mx + monitor.size().width as i32 - win_size.width as i32;
            let max_y = my + monitor.size().height as i32 - win_size.height as i32;

            let target_x = rand_range(&mut rng, mx, max_x);
            let target_y = rand_range(&mut rng, my, max_y);

            let _ = app.emit(
                "wander-start",
                serde_json::json!({ "x": target_x, "y": target_y }),
            );
            smooth_move_to(&win, target_x, target_y, &enabled);
        }
    });
}

/// Enable or disable the automatic wander behaviour.
#[tauri::command]
pub fn set_wander_enabled(enabled: bool, state: State<WanderState>) {
    state.enabled.store(enabled, Ordering::Relaxed);
}

/// Returns whether wander is currently enabled.
#[tauri::command]
pub fn get_wander_enabled(state: State<WanderState>) -> bool {
    state.enabled.load(Ordering::Relaxed)
}

/// Skip the current sleep and trigger a wander move immediately.
#[tauri::command]
pub fn wander_now(state: State<WanderState>) {
    state.trigger_now.store(true, Ordering::Relaxed);
}
