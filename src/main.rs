use std::fs::{self, File};
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use dirs_next;
use rand::Rng;
use serde_json::Value;
use uuid::Uuid;
use std::process::Command;

/// 生成 64 位十六进制字符串
fn generate_hex_64() -> String {
    let mut rng = rand::thread_rng();
    (0..64).map(|_| format!("{:x}", rng.gen_range(0..16))).collect()
}

/// 查找 storage.json 文件路径
fn find_storage_file() -> Option<PathBuf> {
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
fn modify_storage_file(file_path: &PathBuf) {
    // 打开文件并读取内容
    let mut file = File::open(file_path).expect("Failed to open storage.json");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read storage.json");

    // 解析 JSON
    let mut json_data: Value =
        serde_json::from_str(&content).expect("Failed to parse storage.json as JSON");

    // 修改字段值
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

    // 写回 JSON 文件
    let mut file = File::create(file_path).expect("Failed to write to storage.json");
    let new_content = serde_json::to_string_pretty(&json_data).expect("Failed to serialize JSON");
    file.write_all(new_content.as_bytes())
        .expect("Failed to write new content to storage.json");
}

/// 将文件设为可写
fn make_writable(file_path: &PathBuf) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(file_path, fs::Permissions::from_mode(0o666))
            .expect("Failed to make file writable");
    }
    #[cfg(windows)]
    {
        Command::new("attrib")
            .args(&["-R", file_path.to_str().unwrap()])
            .status()
            .expect("Failed to make file writable on Windows");
    }
}

/// 将文件设为只读
fn make_readonly(file_path: &PathBuf) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(file_path, fs::Permissions::from_mode(0o444))
            .expect("Failed to make file readonly");
    }
    #[cfg(windows)]
    {
        Command::new("attrib")
            .args(&["+R", file_path.to_str().unwrap()])
            .status()
            .expect("Failed to make file readonly on Windows");
    }
}

/// 重启 Cursor
fn restart_cursor() {
    #[cfg(target_os = "macos")]
    {
        Command::new("pkill")
            .arg("-f")
            .arg("Cursor")
            .status()
            .expect("Failed to stop Cursor");

        Command::new("open")
            .arg("-a")
            .arg("Cursor")
            .status()
            .expect("Failed to start Cursor");
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("taskkill")
            .args(&["/IM", "Cursor.exe", "/F"])
            .status()
            .expect("Failed to stop Cursor");

        Command::new("start")
            .arg("Cursor")
            .status()
            .expect("Failed to start Cursor");
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("pkill")
            .arg("-f")
            .arg("Cursor")
            .status()
            .expect("Failed to stop Cursor");

        Command::new("Cursor")
            .status()
            .expect("Failed to start Cursor");
    }
}

fn main() {
    if let Some(storage_file) = find_storage_file() {
        println!("Found storage.json at {:?}", storage_file);

        make_writable(&storage_file);
        modify_storage_file(&storage_file);
        make_readonly(&storage_file);

        restart_cursor();
    } else {
        eprintln!("storage.json file not found.");
    }
}