use std::path::PathBuf;

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use waifu_shared::NodeSkillSync;

use super::registry;

pub async fn perform_sync(msg: &NodeSkillSync) -> Result<()> {
    let skills_dir = registry::skills_dir();
    let skill_dir = skills_dir.join(&msg.skill_id);

    tokio::fs::create_dir_all(&skill_dir).await?;

    let client = reqwest::Client::new();
    let mut synced_files: Vec<(String, String)> = Vec::new();

    for file_entry in &msg.files {
        let local_path = skill_dir.join(&file_entry.path);

        if local_sha256(&local_path).await.as_deref() == Some(&file_entry.sha256) {
            log::debug!(
                "skill {}/{} up-to-date, skip",
                msg.skill_id,
                file_entry.path
            );
            synced_files.push((file_entry.path.clone(), file_entry.sha256.clone()));
            continue;
        }

        let url = format!(
            "{}/skills/{}/files/{}",
            msg.base_url.trim_end_matches('/'),
            msg.skill_id,
            file_entry.path
        );
        log::info!("downloading {url}");

        let bytes = client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("GET {url}"))?
            .error_for_status()
            .with_context(|| format!("HTTP error for {url}"))?
            .bytes()
            .await
            .with_context(|| format!("reading body for {url}"))?;

        let actual = hex_sha256(&bytes);
        anyhow::ensure!(
            actual == file_entry.sha256,
            "checksum mismatch for {}: expected {} got {}",
            file_entry.path,
            file_entry.sha256,
            actual
        );

        if let Some(parent) = local_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::write(&local_path, &bytes)
            .await
            .with_context(|| format!("writing {}", local_path.display()))?;

        log::debug!("saved {}", local_path.display());
        synced_files.push((file_entry.path.clone(), file_entry.sha256.clone()));
    }

    registry::write_manifest(&skill_dir, &msg.skill_id, &msg.version, &synced_files).await?;

    Ok(())
}

async fn local_sha256(path: &PathBuf) -> Option<String> {
    let data = tokio::fs::read(path).await.ok()?;
    Some(hex_sha256(&data))
}

fn hex_sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
