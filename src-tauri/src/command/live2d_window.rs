use std::{
    fs,
    net::TcpListener,
    path::{Path, PathBuf},
    sync::Arc,
    thread,
};

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};
use parking_lot::RwLock;
use percent_encoding::{NON_ALPHANUMERIC, percent_decode_str, utf8_percent_encode};
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};

pub struct Live2DServer {
    pub port: u16,
    pub model_dir: Arc<RwLock<Option<PathBuf>>>,
    pub last_model_url: Arc<RwLock<Option<String>>>,
}

pub fn start_live2d_file_server() -> anyhow::Result<Live2DServer> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    let model_dir: Arc<RwLock<Option<PathBuf>>> = Arc::new(RwLock::new(None));
    let model_dir_clone = Arc::clone(&model_dir);

    thread::spawn(move || {
        actix_web::rt::System::new().block_on(async move {
            HttpServer::new(move || {
                let dir = Arc::clone(&model_dir_clone);
                App::new()
                    .app_data(web::Data::new(dir))
                    // match all methods so OPTIONS preflight is handled inside serve_file
                    .route("/{path:.*}", web::route().to(serve_file))
            })
            .listen(listener)
            .expect("failed to bind Live2D file server")
            .run()
            .await
            .expect("Live2D file server error");
        });
    });

    Ok(Live2DServer {
        port,
        model_dir,
        last_model_url: Arc::new(RwLock::new(None)),
    })
}

const CORS_ORIGIN: (&str, &str) = ("Access-Control-Allow-Origin", "*");
const CORS_PRIVATE: (&str, &str) = ("Access-Control-Allow-Private-Network", "true");

async fn serve_file(
    req: HttpRequest,
    model_dir: web::Data<Arc<RwLock<Option<PathBuf>>>>,
) -> HttpResponse {
    if req.method() == actix_web::http::Method::OPTIONS {
        return HttpResponse::NoContent()
            .insert_header(CORS_ORIGIN)
            .insert_header(("Access-Control-Allow-Methods", "GET, OPTIONS"))
            .insert_header(("Access-Control-Allow-Headers", "*"))
            .insert_header(CORS_PRIVATE)
            .finish();
    }

    let raw_path = req.match_info().query("path");
    let decoded = percent_decode_str(raw_path).decode_utf8_lossy();
    let path_str = decoded.as_ref();

    if path_str
        .split(['/', '\\'])
        .any(|seg| seg == ".." || seg == ".")
    {
        return HttpResponse::Forbidden()
            .insert_header(CORS_ORIGIN)
            .body("Forbidden");
    }

    let guard = model_dir.read();
    let Some(base_dir) = guard.as_ref() else {
        return HttpResponse::ServiceUnavailable()
            .insert_header(CORS_ORIGIN)
            .body("Model directory not set");
    };

    let file_path = base_dir.join(path_str);
    match fs::read(&file_path) {
        Ok(data) => HttpResponse::Ok()
            .insert_header(CORS_ORIGIN)
            .insert_header(CORS_PRIVATE)
            .content_type(mime_from_path(&file_path))
            .body(data),
        Err(_) => HttpResponse::NotFound()
            .insert_header(CORS_ORIGIN)
            .body("Not Found"),
    }
}

fn mime_from_path(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("wav") => "audio/wav",
        Some("ogg") => "audio/ogg",
        Some("mp3") => "audio/mpeg",
        _ => "application/octet-stream",
    }
}

fn find_model_file(dir: &Path) -> Option<PathBuf> {
    let entries = fs::read_dir(dir).ok()?;
    let mut subdirs = vec![];

    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_file() {
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name.ends_with(".model3.json")
                || name.ends_with(".model.json")
                || name == "index.json"
            {
                return Some(p);
            }
        } else if p.is_dir() {
            subdirs.push(p);
        }
    }

    for subdir in subdirs {
        if let Some(found) = find_model_file(&subdir) {
            return Some(found);
        }
    }

    None
}

#[tauri::command]
pub fn set_model_path(path: String, app: AppHandle) -> Result<(), String> {
    let server = app.state::<Live2DServer>();
    let input = PathBuf::from(&path);

    let model_path = if input.is_dir() {
        find_model_file(&input).ok_or_else(|| format!("No model JSON found in: {path}"))?
    } else if input.is_file() {
        input
    } else {
        return Err(format!("Path does not exist: {path}"));
    };

    let model_dir = model_path
        .parent()
        .map(PathBuf::from)
        .ok_or_else(|| format!("Cannot determine parent directory of: {path}"))?;

    let filename = model_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| format!("Cannot determine filename of: {path}"))?
        .to_string();

    *server.model_dir.write() = Some(model_dir);

    let encoded = utf8_percent_encode(&filename, NON_ALPHANUMERIC).to_string();
    let model_url = format!("http://127.0.0.1:{}/{}", server.port, encoded);

    *server.last_model_url.write() = Some(model_url.clone());

    app.emit("load-model", model_url).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_current_model_url(app: AppHandle) -> Option<String> {
    let server = app.state::<Live2DServer>();
    server.last_model_url.read().clone()
}

#[tauri::command]
pub fn open_settings_window(app: AppHandle) -> Result<(), String> {
    use tauri::WebviewUrl;
    match app.get_webview_window("settings") {
        Some(win) => {
            win.show().map_err(|e| e.to_string())?;
            win.set_focus().map_err(|e| e.to_string())
        }
        None => {
            tauri::WebviewWindowBuilder::new(&app, "settings", WebviewUrl::App("settings".into()))
                .title("設定")
                .inner_size(860.0, 600.0)
                .build()
                .map(|_| ())
                .map_err(|e| e.to_string())
        }
    }
}

#[tauri::command]
pub fn open_live2d_window(app: AppHandle) -> Result<(), String> {
    let win = app
        .get_webview_window("live2d-preview")
        .ok_or_else(|| "live2d-preview window not found".to_string())?;
    win.show().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn close_live2d_window(app: AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window("live2d-preview") {
        win.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[derive(Serialize)]
pub struct ReadFileResult {
    pub data: Vec<u8>,
    pub err: String,
}

/// Parses the currently loaded model3.json and returns expression names.
#[tauri::command]
pub fn list_expressions(app: AppHandle) -> Result<Vec<String>, String> {
    let server = app.state::<Live2DServer>();
    let guard = server.model_dir.read();
    let base = guard.as_ref().ok_or("model not loaded")?;

    // Find *.model3.json inside base
    let model_file = find_model_file(base).ok_or("no model3.json found")?;
    let data = fs::read_to_string(&model_file).map_err(|e| e.to_string())?;
    let json: serde_json::Value = serde_json::from_str(&data).map_err(|e| e.to_string())?;

    let expressions = json
        .pointer("/FileReferences/Expressions")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|e| e.get("Name").and_then(|n| n.as_str()).map(str::to_owned))
                .collect()
        })
        .unwrap_or_default();

    Ok(expressions)
}

#[tauri::command]
pub fn read_file(file_path: PathBuf) -> ReadFileResult {
    match fs::read(&file_path) {
        Ok(data) => ReadFileResult {
            data,
            err: String::new(),
        },
        Err(e) => ReadFileResult {
            data: vec![],
            err: e.to_string(),
        },
    }
}
