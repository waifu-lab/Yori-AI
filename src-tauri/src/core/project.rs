use std::{fs, sync::{Arc, OnceLock}, thread};

use notify::Watcher;
use parking_lot::{Mutex, RwLock};
use tauri::plugin;

use super::state::AppConsts;
use crate::{schema::{PluginConfig, Profile, ProjectMeta}, utils::path};

pub struct Project {
    meta: ProjectMeta,
    profile: Profile,
    plugins: RwLock<Vec<PluginConfig>>,
    watcher: OnceLock<notify::RecommendedWatcher>,
    watcher_thread: Mutex<OnceLock<thread::JoinHandle<()>>>,
    drop: OnceLock<()>,
}

impl Project {
    pub fn new(meta: ProjectMeta) -> anyhow::Result<Self> {
        let project_dir = meta.data_root.clone();
        let profile_path = path::project_profile_path(project_dir);
        let assets_path = path::project_assets_path(meta.data_root.clone());
        if !profile_path.exists() || !assets_path.exists() {
            return Err(anyhow::anyhow!(
                "Project is missing required files or directories, at\n\t{}\n\t{}",
                profile_path.display(), assets_path.display()
            ));
        }

        let profile_content = fs::read_to_string(profile_path)?;
        let profile = serde_json::from_str(&profile_content)?;

        // todo: load plugins
        Ok(Self {
            meta,
            profile,
            plugins: RwLock::new(Vec::new()),
            watcher: OnceLock::new(),
            watcher_thread: Mutex::new(OnceLock::new()),
            drop: OnceLock::new(),
        })
    }

    pub fn initialize(self: Arc<Self>) -> anyhow::Result<()> {
        let project_dir = self.meta.data_root.clone();
        let (tx, rx) = crossbeam_channel::unbounded();
        let mut watcher = notify::recommended_watcher(move |res| tx.send(res).unwrap())?;
        let cloned_self = self.clone();
        let watcher_thread = thread::spawn(move || {
            loop {
                if cloned_self.drop.get().is_some() {
                    break;
                }

                match rx.try_recv() {
                    Ok(event) => cloned_self.watcher_handler(event),
                    Err(crossbeam_channel::TryRecvError::Empty) => (),
                    Err(crossbeam_channel::TryRecvError::Disconnected) => break,
                }
            }
        });

        watcher.watch(&project_dir, notify::RecursiveMode::Recursive)?;
        self.watcher.set(watcher).unwrap();
        self.watcher_thread.lock().set(watcher_thread).unwrap();
        Ok(())
    }

    pub fn destroy(&self) {
        if self.drop.set(()).is_ok() {
            if let Some(handle) = self.watcher_thread.lock().take() {
                handle.join().unwrap();
            }
        }
    }

    fn watcher_handler(&self, _event: notify::Result<notify::Event>) {
        todo!("Implement file change handling logic");
    }
}

pub fn create_project(consts: Arc<AppConsts>, meta: ProjectMeta) -> anyhow::Result<Project> {
    let meta_path = path::project_file_path(consts.app_root(), &meta.name);
    let profile_path = path::project_profile_path(meta.data_root.clone());
    let assets_path = path::project_assets_path(meta.data_root.clone());
    let plugins_path = path::plugin_dir_path(meta.data_root.clone());
    fs::create_dir_all(assets_path)?;
    fs::create_dir_all(plugins_path)?;

    let profile = Profile::default();
    let profile_content = serde_json::to_string_pretty(&profile)?;
    let meta_content = serde_json::to_string_pretty(&meta)?;
    fs::write(profile_path, profile_content)?;
    fs::write(meta_path, meta_content)?;
    Project::new(meta)
}

pub fn load_project(state: Arc<AppConsts>, project_name: &str) -> anyhow::Result<Project> {
    let file_path = path::project_file_path(state.app_root(), project_name);
    let content = fs::read_to_string(file_path)?;
    let project_meta = serde_json::from_str(&content)?;
    Project::new(project_meta)
}