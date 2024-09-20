// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sysinfo::{ Pid, Process, System };

#[derive(serde::Serialize)]
struct SystemProcess {
    pid: u32,
    name: String,
    cpu: f32,
    memory: u64,
    disk_read: u64,
    disk_write: u64,
}

impl SystemProcess {
    pub fn new(pid: &Pid, process: &Process) -> Self {
        return Self {
            pid: pid.as_u32(),
            name: process.name().to_str().unwrap().to_string(),
            cpu: process.cpu_usage(),
            memory: process.memory(),
            disk_read: process.disk_usage().total_read_bytes,
            disk_write: process.disk_usage().total_written_bytes,
        };
    }
}

#[derive(serde::Serialize)]
struct SysInfo {
    total_memory: u64,
    used_memory: u64,
    sys_name: Option<String>,
    kernel_version: Option<String>,
    os_version: Option<String>,
    host_name: Option<String>,
    cpus: usize,
    processes: Vec<SystemProcess>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet() -> SysInfo {
    let sys = System::new_all();
    let mut processes = Vec::<SystemProcess>::new();
    for (pid, process) in sys.processes() {
        processes.push(SystemProcess::new(pid, process));
    }
    SysInfo {
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        sys_name: System::name(),
        kernel_version: System::kernel_version(),
        os_version: System::os_version(),
        host_name: System::host_name(),
        cpus: sys.cpus().len(),
        processes
    }
}

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
