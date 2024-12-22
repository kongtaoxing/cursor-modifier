use dirs_next;
use rand::Rng;
use serde_json::Value;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use uuid::Uuid;

/// 生成 64 位十六进制字符串
pub fn generate_hex_64() -> String {
    let mut rng = rand::thread_rng();
    (0..64)
        .map(|_| format!("{:x}", rng.gen_range(0..16)))
        .collect()
}

/// 查找 storage.json 文件路径
pub fn find_storage_file() -> Option<PathBuf> {
    let base_dir = dirs_next::config_dir()
        .or_else(|| dirs_next::home_dir())
        .unwrap_or_else(|| PathBuf::from("."));
    let storage_path = base_dir.join("Cursor/User/globalStorage/storage.json");

    if storage_path.exists() {
        Some(storage_path)
    } else {
        eprintln!("storage.json not found at {:?}", storage_path);
        None
    }
}

/// 修改 JSON 文件
pub fn modify_storage_file(file_path: &PathBuf) {
    let mut file = File::open(file_path).expect("Failed to open storage.json");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read storage.json");

    let mut json_data: Value =
        serde_json::from_str(&content).expect("Failed to parse storage.json as JSON");

    if let Some(map) = json_data.as_object_mut() {
        map.insert(
            "telemetry.macMachineId".to_string(),
            Value::String(generate_hex_64()),
        );
        map.insert(
            "telemetry.machineId".to_string(),
            Value::String(generate_hex_64()),
        );
        map.insert(
            "telemetry.devDeviceId".to_string(),
            Value::String(Uuid::new_v4().to_string()),
        );
    } else {
        eprintln!("Expected a JSON object at the root of storage.json");
        return;
    }

    let mut file = File::create(file_path).expect("Failed to write to storage.json");
    let new_content = serde_json::to_string_pretty(&json_data).expect("Failed to serialize JSON");
    file.write_all(new_content.as_bytes())
        .expect("Failed to write new content to storage.json");
}

/// 将文件设为可写
pub fn make_writable(file_path: &PathBuf) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(file_path, fs::Permissions::from_mode(0o666))
            .expect("Failed to make file writable");
    }
    #[cfg(windows)]
    {
        use std::process::Command;
        Command::new("attrib")
            .args(&["-R", file_path.to_str().unwrap()])
            .status()
            .expect("Failed to make file writable on Windows");
    }
}

/// 将文件设为只读
pub fn make_readonly(file_path: &PathBuf) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(file_path, fs::Permissions::from_mode(0o444))
            .expect("Failed to make file readonly");
    }
    #[cfg(windows)]
    {
        use std::process::Command;
        Command::new("attrib")
            .args(&["+R", file_path.to_str().unwrap()])
            .status()
            .expect("Failed to make file readonly on Windows");
    }
}
