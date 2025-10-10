use std::fs;

use serde::Serialize;
use tauri::{Manager, Runtime};

use crate::{error::{PathError, SerializeError}, types::Project, models::ProjectImpl};

pub fn save_project<R: Runtime, P: Project + Serialize>(app: &impl Manager<R>, project: &P) -> anyhow::Result<()> {
    let config_dir = app.path().config_dir()
        .or(Err(PathError::InvalidPath))?;
    let file_path = config_dir
        .join("projects")
        .join(format!("{}.proj", project.name()));
    fs::write(file_path, bincode::serialize(project)
        .map_err(SerializeError::SerializationFailed)?)?;
    Ok(())
}

pub fn load_projects<R: Runtime>(app: &impl Manager<R>) -> anyhow::Result<Vec<Box<dyn Project>>> {
    let config_dir = app.path().config_dir()
        .or(Err(PathError::InvalidPath))?;
    let projects_dir = config_dir.join("projects");
    if !projects_dir.exists() {
        fs::create_dir_all(&projects_dir)
            .map_err(PathError::IoError)?;
        return Ok(vec![]);
    }

    let mut projects: Vec<Box<dyn Project>> = vec![];
    for entry in fs::read_dir(projects_dir)
        .map_err(PathError::IoError)? {
        let entry = entry.map_err(PathError::IoError)?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("proj") {
            let data = fs::read(&path)
                .map_err(PathError::IoError)?;
            let project: ProjectImpl = bincode::deserialize(&data)
                .map_err(SerializeError::DeserializationFailed)?;
            projects.push(Box::new(project));
        }
    }
    
    Ok(projects)
}