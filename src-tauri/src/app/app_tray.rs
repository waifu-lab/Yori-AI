use tauri::{
    App, Result, Runtime,
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
};

pub fn build_menu<R: Runtime>(app: &App<R>) -> Result<Menu<R>> {
    let menu = Menu::new(app)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, Option::<&str>::None)?;
    menu.append(&quit)?;
    Ok(menu)
}

pub fn build_tray<R: Runtime>(app: &App<R>) -> Result<TrayIcon<R>> {
    let tray_menu = build_menu(app)?;

    TrayIconBuilder::new()
        .menu(&tray_menu)
        .on_menu_event(|app, event| {
            if event.id().as_ref() == "quit" {
                app.exit(0);
            }
        })
        .build(app)
}
