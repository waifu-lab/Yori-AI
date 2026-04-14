use serde_json::{json, Value};
use sysinfo::{Disks, Networks, System};
use waifu_shared::NodeError;

pub async fn sysinfo(_params: &Value) -> Result<Value, NodeError> {
    tokio::task::spawn_blocking(collect_sysinfo)
        .await
        .map_err(|e| NodeError {
            code: "INVALID_ACTION".into(),
            message: e.to_string(),
        })?
}

fn collect_sysinfo() -> Result<Value, NodeError> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let mem_total_mb = sys.total_memory() / 1024 / 1024;
    let mem_used_mb = sys.used_memory() / 1024 / 1024;

    let disks = Disks::new_with_refreshed_list();
    let disk_list: Vec<Value> = disks
        .iter()
        .map(|d| {
            json!({
                "name": d.name().to_string_lossy(),
                "mount": d.mount_point().to_string_lossy(),
                "total_mb": d.total_space() / 1024 / 1024,
                "available_mb": d.available_space() / 1024 / 1024
            })
        })
        .collect();

    let networks = Networks::new_with_refreshed_list();
    let net_list: Vec<Value> = networks
        .iter()
        .map(|(name, data)| {
            json!({
                "interface": name,
                "received_bytes": data.received(),
                "transmitted_bytes": data.transmitted()
            })
        })
        .collect();

    let os_name = System::name().unwrap_or_default();
    let os_version = System::os_version().unwrap_or_default();
    let hostname = System::host_name().unwrap_or_default();

    Ok(json!({
        "cpu_usage": cpu_usage,
        "mem_total_mb": mem_total_mb,
        "mem_used_mb": mem_used_mb,
        "disks": disk_list,
        "network": net_list,
        "os": format!("{os_name} {os_version}"),
        "hostname": hostname
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn sysinfo_returns_expected_shape_or_known_error() {
        match sysinfo(&json!({})).await {
            Ok(value) => {
                assert!(value.get("cpu_usage").is_some());
                assert!(value.get("mem_total_mb").is_some());
                assert!(value.get("mem_used_mb").is_some());
                assert!(value.get("disks").is_some());
                assert!(value.get("network").is_some());
                assert!(value.get("os").is_some());
                assert!(value.get("hostname").is_some());
            }
            Err(err) => {
                assert_eq!(err.code, "INVALID_ACTION");
            }
        }
    }
}
