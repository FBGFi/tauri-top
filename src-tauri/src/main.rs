// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sysinfo::System;

#[derive(Debug, serde::Serialize)]
struct ProcessInfo {
  name: String,
  memory: u64,
  cpu_usage: f32,
  pid: i32,
}

#[tauri::command]
fn greet() -> Vec<ProcessInfo> {
  let mut sys: System = System::new_all();

  sys.refresh_all();

  return sys
    .processes()
    .iter()
    .map(|(&pid, process)| ProcessInfo {
      name: process.name().to_str().unwrap_or("Unknown").to_string(),
      memory: process.memory(),
      cpu_usage: process.cpu_usage(),
      pid: pid.as_u32() as i32,
    })
    .collect();
}

fn main() {
  tauri::Builder
    ::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
