// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{ thread, time };

use serde::Serialize;
use sysinfo::System;
use tauri::{ async_runtime,  AppHandle, Manager };

#[derive(Serialize, Clone)]
struct SystemInfo {
    system_name: Option<String>,
    kernel_version: Option<String>,
    os_version: Option<String>,
    host_name: Option<String>,
    memory_usage: u64,
    cpu_usage: f32,
}

fn update_system_info(app: &AppHandle) {
    let mut sys = System::new_all();
    sys.refresh_cpu_usage();
    app.emit_all("update_system_info", SystemInfo {
        system_name: System::name(),
        kernel_version: System::kernel_version(),
        os_version: System::os_version(),
        host_name: System::host_name(),
        memory_usage: sys.used_memory(),
        cpu_usage: sys.global_cpu_usage(),
    }).unwrap();
}

fn main() {
    tauri::Builder
        ::default()
        .setup(|app| {
            let handle = app.handle();
            async_runtime::spawn(async move {
                loop {
                    update_system_info(&handle);
                    thread::sleep(time::Duration::from_millis(1000));
                }
            });
            Ok(())
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
