use serde::Deserialize;
use std::io::Write;
use std::path::PathBuf;

const GITHUB_API: &str = "https://api.github.com/repos/waifu-lab/Yori-AI/releases/latest";

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

fn asset_name() -> &'static str {
    if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
        "waifu-node-x86_64-pc-windows-msvc.exe"
    } else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        "waifu-node-aarch64-apple-darwin"
    } else if cfg!(all(target_os = "macos", target_arch = "x86_64")) {
        "waifu-node-x86_64-apple-darwin"
    } else if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        "waifu-node-x86_64-unknown-linux-gnu"
    } else {
        "waifu-node"
    }
}

fn dest_path() -> anyhow::Result<PathBuf> {
    let exe = std::env::current_exe()?;
    let dir = exe
        .parent()
        .ok_or_else(|| anyhow::anyhow!("no parent dir"))?;
    let bin = if cfg!(target_os = "windows") {
        dir.join("waifu-node.exe")
    } else {
        dir.join("waifu-node")
    };
    Ok(bin)
}

pub async fn latest_version() -> anyhow::Result<String> {
    let client = reqwest::Client::builder().user_agent("Yori-AI").build()?;
    let release: Release = client.get(GITHUB_API).send().await?.json().await?;
    Ok(release.tag_name)
}

/// Download the latest release binary and place it next to this exe.
/// Calls `progress_cb(downloaded_bytes, total_bytes)` during download.
pub async fn download_latest<F>(progress_cb: F) -> anyhow::Result<PathBuf>
where
    F: Fn(u64, u64) + Send + 'static,
{
    let client = reqwest::Client::builder().user_agent("Yori-AI").build()?;

    let release: Release = client.get(GITHUB_API).send().await?.json().await?;
    let target = asset_name();

    let asset = release
        .assets
        .iter()
        .find(|a| a.name == target)
        .ok_or_else(|| anyhow::anyhow!("no asset '{target}' in release {}", release.tag_name))?;

    log::info!(
        "downloading {} from {}",
        asset.name,
        asset.browser_download_url
    );

    let resp = client.get(&asset.browser_download_url).send().await?;
    let total = resp.content_length().unwrap_or(0);

    let dest = dest_path()?;
    let tmp = dest.with_extension("tmp");

    {
        use futures_util::StreamExt;
        let mut file = std::fs::File::create(&tmp)?;
        let mut downloaded: u64 = 0;
        let mut stream = resp.bytes_stream();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk)?;
            downloaded += chunk.len() as u64;
            progress_cb(downloaded, total);
        }
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&tmp, std::fs::Permissions::from_mode(0o755))?;
    }

    std::fs::rename(&tmp, &dest)?;
    log::info!("waifu-node installed to {}", dest.display());
    Ok(dest)
}
