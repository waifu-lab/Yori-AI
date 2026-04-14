use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use waifu_shared::InstalledSkill;

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalManifest {
    pub skill_id: String,
    pub version: String,
    pub synced_at: String,
    pub files: Vec<ManifestFile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManifestFile {
    pub path: String,
    pub sha256: String,
}

pub fn skills_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".into());
        PathBuf::from(appdata).join("waifu").join("skills")
    }
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        PathBuf::from(home)
            .join("Library")
            .join("Application Support")
            .join("waifu")
            .join("skills")
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        PathBuf::from(home)
            .join(".local")
            .join("share")
            .join("waifu")
            .join("skills")
    }
}

pub async fn scan_installed(dir: &PathBuf) -> Vec<InstalledSkill> {
    let mut result = Vec::new();

    let mut entries = match tokio::fs::read_dir(dir).await {
        Ok(e) => e,
        Err(_) => return result,
    };

    while let Ok(Some(entry)) = entries.next_entry().await {
        let manifest_path = entry.path().join(".manifest.json");
        if let Ok(data) = tokio::fs::read_to_string(&manifest_path).await {
            if let Ok(manifest) = serde_json::from_str::<LocalManifest>(&data) {
                result.push(InstalledSkill {
                    skill_id: manifest.skill_id,
                    version: manifest.version,
                });
            }
        }
    }

    result
}

pub async fn write_manifest(
    dir: &PathBuf,
    skill_id: &str,
    version: &str,
    files: &[(String, String)], // (path, sha256)
) -> anyhow::Result<()> {
    let manifest = LocalManifest {
        skill_id: skill_id.to_string(),
        version: version.to_string(),
        synced_at: chrono_now(),
        files: files
            .iter()
            .map(|(p, s)| ManifestFile {
                path: p.clone(),
                sha256: s.clone(),
            })
            .collect(),
    };

    let json = serde_json::to_string_pretty(&manifest)?;
    tokio::fs::create_dir_all(dir).await?;
    tokio::fs::write(dir.join(".manifest.json"), json).await?;
    Ok(())
}

fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    format!("{secs}")
}
