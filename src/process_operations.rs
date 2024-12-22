use std::process::Command;

/// 检查 Cursor 是否正在运行
pub fn is_cursor_running() -> bool {
    #[cfg(target_os = "macos")]
    {
        Command::new("pgrep")
            .arg("-f")
            .arg("Cursor")
            .output()
            .map(|output| !output.stdout.is_empty())
            .unwrap_or(false)
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("tasklist")
            .args(&["/FI", "IMAGENAME eq Cursor.exe", "/NH"])
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).contains("Cursor.exe"))
            .unwrap_or(false)
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("pgrep")
            .arg("-f")
            .arg("Cursor")
            .output()
            .map(|output| !output.stdout.is_empty())
            .unwrap_or(false)
    }
}

/// 关闭 Cursor
pub fn close_cursor() {
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        Command::new("pkill")
            .arg("-f")
            .arg("Cursor")
            .status()
            .expect("Failed to stop Cursor");
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("taskkill")
            .args(&["/IM", "Cursor.exe", "/F"])
            .status()
            .expect("Failed to stop Cursor");
    }
}

/// 启动 Cursor
pub fn start_cursor() {
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg("-a")
            .arg("Cursor")
            .status()
            .expect("Failed to start Cursor");
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("start")
            .arg("Cursor")
            .status()
            .expect("Failed to start Cursor");
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("Cursor")
            .status()
            .expect("Failed to start Cursor");
    }
}
