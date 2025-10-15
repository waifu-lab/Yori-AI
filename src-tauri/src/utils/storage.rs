use std::fs;

use tauri::{Manager, Runtime};

use crate::{error::BackendError, domain::ProjectData};

pub fn save_project<R: Runtime>(app: &impl Manager<R>, project: &ProjectData) -> anyhow::Result<()> {
    let data_dir = app.path().data_dir()
        .or(Err(BackendError::InvalidPath))?;
    let file_path = data_dir
        .join("projects")
        .join(format!("{}.proj", project.name()));
    fs::write(file_path, bincode::serialize(project)
        .map_err(BackendError::SerializationFailed)?)?;
    Ok(())
}

pub fn load_project<R: Runtime>(app: &impl Manager<R>, name: &str) -> anyhow::Result<ProjectData> {
    let data_dir = app.path().data_dir()
        .or(Err(BackendError::InvalidPath))?;
    let file_path = data_dir
        .join("projects")
        .join(format!("{}.proj", name));
    let data = fs::read(file_path)
        .map_err(BackendError::IoError)?;
    let project: ProjectData = bincode::deserialize(&data)
        .map_err(BackendError::DeserializationFailed)?;
    Ok(project)
}

pub fn load_projects<R: Runtime>(app: &impl Manager<R>) -> anyhow::Result<Vec<ProjectData>> {
    let data_dir = app.path().data_dir()
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