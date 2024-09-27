// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn generate_header_text(operating_system: &str) -> String {
    format!("System info for {}", operating_system)
}

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![generate_header_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
