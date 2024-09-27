# Using Rust for low level operations

Previous example returned simple formatted string from the backend, but we can instead run low level operations and access resources that are normally outside of web applications scope.

## Installing Rust dependencies

Installing dependencies for the Rust backend is done via cargo in src-tauri directory. For this example we use [sysinfo](https://docs.rs/sysinfo/latest/sysinfo/) package to return current system resources.

- Run ```cargo add sysinfo``` in src-tauri directory