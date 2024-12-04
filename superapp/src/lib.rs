use std::path::Path;

use tauri::{command, AppHandle, Builder, WebviewUrl, WebviewWindowBuilder};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[command]
fn open(handle: AppHandle, name: &str) {
    let _wapp_window = WebviewWindowBuilder::new(&handle, name, WebviewUrl::App(Path::new(name).join("index.html")))
        .build()
        .expect("Failed to open docs");
}

#[cfg_attr(mobile, mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, open])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
