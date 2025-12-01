use std::path::PathBuf;

pub fn project_default_path(app_dir: PathBuf, project_name: &str) -> PathBuf {
    app_dir
        .join("default/projects")
        .join(project_name)
}

pub fn project_file_path(app_dir: PathBuf, project_name: &str) -> PathBuf {
    app_dir
        .join("projects")
        .join(format!("{}.yoriproj", project_name))
}

pub fn project_assets_path(project_dir: PathBuf) -> PathBuf {
    project_dir
        .join("assets")
}

pub fn project_profile_path(project_dir: PathBuf) -> PathBuf {
    project_dir
        .join("profile.json")
}

pub fn plugin_dir_path(project_dir: PathBuf) -> PathBuf {
    project_dir
        .join("plugins")
}

pub fn plugin_config_path(project_dir: PathBuf, plugin_id: &str) -> PathBuf {
    plugin_dir_path(project_dir)
        .join(plugin_id)
        .join("plugin.toml")
}