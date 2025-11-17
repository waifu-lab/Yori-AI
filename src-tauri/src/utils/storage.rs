use std::{fs, path::{Path, PathBuf}};

use plugin_lib::types::Plugin;
use tauri::{AppHandle, Manager};

use crate::{schema::{DynamicPlugin, ProjectData}, error::BackendError};

pub fn save_project(app: &AppHandle, project: &ProjectData) -> anyhow::Result<()> {
    let data_dir = app.path().app_data_dir()
        .or(Err(BackendError::InvalidPath))?;
    let projects_dir = data_dir.join("projects");
    if !projects_dir.exists() {
        fs::create_dir_all(&projects_dir)
            .map_err(BackendError::IoError)?;
    }
    let file_path = projects_dir
        .join(format!("{}.proj", project.name()));
    fs::write(file_path, bincode::serialize(project)
        .map_err(BackendError::SerializationFailed)?)?;
    Ok(())
}

pub fn delete_project(app: &AppHandle, project_name: &str) -> anyhow::Result<()> {
    let data_dir = app.path().app_data_dir()
        .or(Err(BackendError::InvalidPath))?;
    let file_path = data_dir
        .join("projects")
        .join(format!("{}.proj", project_name));
    if file_path.exists() {
        fs::remove_file(file_path)
            .map_err(BackendError::IoError)?;
    }
    Ok(())
}

// pub fn load_project<R: Runtime>(app: &impl Manager<R>, name: &str) -> anyhow::Result<ProjectData> {
//     let data_dir = app.path().data_dir()
//         .or(Err(BackendError::InvalidPath))?;
//     let file_path = data_dir
//         .join("projects")
//         .join(format!("{}.proj", name));
//     let data = fs::read(file_path)
//         .map_err(BackendError::IoError)?;
//     let project: ProjectData = bincode::deserialize(&data)
//         .map_err(BackendError::DeserializationFailed)?;
//     Ok(project)
// }

pub fn load_projects(app: &AppHandle) -> anyhow::Result<Vec<ProjectData>> {
    let data_dir = app.path().app_data_dir()
        .or(Err(BackendError::InvalidPath))?;
    let projects_dir = data_dir.join("projects");
    if !projects_dir.exists() {
        fs::create_dir_all(&projects_dir)
            .map_err(BackendError::IoError)?;
        return Ok(vec![]);
    }

    let mut projects: Vec<ProjectData> = vec![];
    for entry in fs::read_dir(projects_dir).map_err(BackendError::IoError)? {
        let entry = entry.map_err(BackendError::IoError)?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("proj") {
            let data = fs::read(&path)
                .map_err(BackendError::IoError)?;
            let project: ProjectData = bincode::deserialize(&data)
                .map_err(BackendError::DeserializationFailed)?;
            projects.push(project);
        }
    }

    Ok(projects)
}

pub fn copy_plugin<P: AsRef<Path>>(app: &AppHandle, plugin_path: P) -> anyhow::Result<PathBuf> {
    let data_dir = app.path().app_data_dir()
        .or(Err(BackendError::InvalidPath))?;
    let plugins_dir = data_dir.join("plugins");
    if !plugins_dir.exists() {
        fs::create_dir_all(&plugins_dir)
            .map_err(BackendError::IoError)?;
    }
    let file_name = plugin_path.as_ref().file_name()
        .ok_or(BackendError::InvalidPath)?;
    let dest_path = plugins_dir.join(file_name);
    fs::copy(plugin_path, dest_path.clone())
        .map_err(BackendError::IoError)?;
    Ok(dest_path)
}

pub fn delete_plugin<P: AsRef<Path>>(app: &AppHandle, file_name: P) -> anyhow::Result<()> {
    let data_dir = app.path().app_data_dir()
        .or(Err(BackendError::InvalidPath))?;
    let plugins_dir = data_dir.join("plugins");
    let plugin_path = plugins_dir.join(file_name);
    if plugin_path.exists() {
        fs::remove_file(plugin_path)
            .map_err(BackendError::IoError)?;
    }
    Ok(())
}

pub fn load_plugin<P: AsRef<Path>>(_app: &AppHandle, plugin_path: P) -> anyhow::Result<Box<dyn Plugin>> {
    #[cfg(windows)]
    let ext = "dll";
    #[cfg(target_os = "linux")]
    let ext = "so";
    #[cfg(target_os = "macos")]
    let ext = "dylib";
    if plugin_path.as_ref().extension().and_then(|s| s.to_str()) != Some(ext) {
        return Err(BackendError::InvalidPluginFormat.into());
    }

    unsafe {
        let lib = libloading::Library::new(plugin_path.as_ref())
            .map_err(BackendError::PluginLoadFailed)?;

        let plugin_constructor: libloading::Symbol<unsafe extern "C" fn() -> *mut dyn Plugin> =
            lib.get(b"_plugin_create")
                .map_err(BackendError::PluginLoadFailed)?;

        let boxed_raw = plugin_constructor();
        let plugin = Box::from_raw(boxed_raw);
        Ok(Box::new(DynamicPlugin::new(plugin, plugin_path.as_ref().to_path_buf(), lib)))
    }
}

pub fn load_plugins(app: &AppHandle) -> anyhow::Result<Vec<Box<dyn Plugin>>> {
    let data_dir = app.path().app_data_dir()
        .or(Err(BackendError::InvalidPath))?;
    let plugins_dir = data_dir.join("plugins");
    if !plugins_dir.exists() {
        fs::create_dir_all(&plugins_dir)
            .map_err(BackendError::IoError)?;
        return Ok(vec![]);
    }

    #[cfg(windows)]
    let ext = "dll";
    #[cfg(target_os = "linux")]
    let ext = "so";
    #[cfg(target_os = "macos")]
    let ext = "dylib";
    let mut plugins: Vec<Box<dyn Plugin>> = vec![];
    for entry in fs::read_dir(plugins_dir).map_err(BackendError::IoError)? {
        let entry = entry.map_err(BackendError::IoError)?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some(ext) {
            unsafe {
                let lib = libloading::Library::new(&path)
                    .map_err(BackendError::PluginLoadFailed)?;

                let plugin_constructor: libloading::Symbol<unsafe extern "C" fn() -> *mut dyn Plugin> =
                    lib.get(b"_plugin_create")
                        .map_err(BackendError::PluginLoadFailed)?;

                let boxed_raw = plugin_constructor();
                let plugin = Box::from_raw(boxed_raw);
                plugins.push(Box::new(DynamicPlugin::new(plugin, path, lib)));
            }
        }
    }

    Ok(plugins)
}