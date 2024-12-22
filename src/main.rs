mod json_operations;
mod process_operations;

use json_operations::{find_storage_file, make_readonly, make_writable, modify_storage_file};
use process_operations::{close_cursor, is_cursor_running, start_cursor};
use std::io::{self, Write};

fn print_banner() {
    println!(r#"
    ╔═══════════════════════════════════════╗
    ║        Cursor Config Modifier         ║
    ║            Version 1.0.0              ║
    ╚═══════════════════════════════════════╝
    "#);
}

fn get_user_input(prompt: &str) -> bool {
    print!("{} (y/n): ", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase() == "y"
}

fn pause() {
    print!("\nPress Enter to continue...");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn main() {
    print_banner();

    // 检查 Cursor 是否在运行
    if is_cursor_running() {
        println!("\n⚠️ Cursor is currently running!");
        if !get_user_input("Do you want to stop Cursor and continue?") {
            println!("\nOperation cancelled. Exiting...");
            return;
        }
        println!("\nStopping Cursor...");
        close_cursor();

        //再次检查 Cursor 是否运行，如果运行则提醒用户手动退出
        if is_cursor_running() {
            println!("\n⚠️ Cursor is still running, Please stop it manually then restart the process..");
            pause();
        }
    }

    // 修改 storage.json
    if let Some(storage_file) = find_storage_file() {
        println!("\nFound storage.json at {:?}", storage_file);

        make_writable(&storage_file);
        modify_storage_file(&storage_file);
        make_readonly(&storage_file);

        println!("\n✅ Configuration updated successfully!");
        
        if get_user_input("\nDo you want to start Cursor now?") {
            println!("\nStarting Cursor...");
            start_cursor();
            println!("Cursor has been started!");
        } else {
            println!("\nCursor was not started.");
        }
    } else {
        eprintln!("\n❌ Error: storage.json file not found.");
    }

    pause();
}
