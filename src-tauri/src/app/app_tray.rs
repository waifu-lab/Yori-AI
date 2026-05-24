use tauri::{
    App, Manager, Result, Runtime, WebviewUrl, WebviewWindowBuilder,
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
};

pub fn build_menu<R: Runtime>(app: &App<R>) -> Result<Menu<R>> {
    let menu = Menu::new(app)?;
    let show = MenuItem::with_id(app, "show", "開啟主介面", true, Option::<&str>::None)?;
    let chat = MenuItem::with_id(app, "chat", "開啟聊天", true, Option::<&str>::None)?;
    let settings = MenuItem::with_id(app, "settings", "設定", true, Option::<&str>::None)?;
    let quit = MenuItem::with_id(app, "quit", "離開", true, Option::<&str>::None)?;
    menu.append(&show)?;
    menu.append(&chat)?;
    menu.append(&settings)?;
    menu.append(&quit)?;
    Ok(menu)
}

fn open_or_create_window<R: Runtime>(
    app: &tauri::AppHandle<R>,
    label: &str,
    url: &str,
    title: &str,
    width: f64,
    height: f64,
) {
    if let Some(window) = app.get_webview_window(label) {
        let _ = window.show();
        let _ = window.set_focus();
    } else {
        let _ = WebviewWindowBuilder::new(app, label, WebviewUrl::App(url.into()))
            .title(title)
            .inner_size(width, height)
            .build();
    }
}

fn open_live2d_window<R: Runtime>(app: &tauri::AppHandle<R>) {
    if let Some(window) = app.get_webview_window("live2d-preview") {
        let _ = window.show();
        let _ = window.set_focus();
    } else {
        let _ = WebviewWindowBuilder::new(app, "live2d-preview", WebviewUrl::App("live2d".into()))
            .title("Live2D")
            .inner_size(400.0, 600.0)
            .transparent(true)
            .decorations(false)
            .shadow(false)
            .always_on_top(true)
            .build();
    }
}

pub fn build_tray<R: Runtime>(app: &App<R>) -> Result<TrayIcon<R>> {
    let tray_menu = build_menu(app)?;

    let mut builder = TrayIconBuilder::new().menu(&tray_menu);
    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }
    builder.on_menu_event(|app, event| match event.id().as_ref() {
            "show" => {
                open_live2d_window(app);
            }
            "chat" => {
                open_or_create_window(app, "chat", "chat", "對話介面", 860.0, 600.0);
            }
            "settings" => {
                open_or_create_window(app, "settings", "settings", "設定", 860.0, 600.0);
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)
}
