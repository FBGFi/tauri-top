# Tauri + React + Typescript

## Prerequisites

[Guide at tauri documentation](https://tauri.app/v1/guides/getting-started/prerequisites)

### Windows

- Install [Node.js](https://nodejs.org/en)
- Install [Visual Studio build tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
  - Select MSVC and Windows 10/11 SDK
- Install [Rust](https://www.rust-lang.org/tools/install)
  - Most likely requires restarting the computer

### MacOS

- Install [Node.js](https://nodejs.org/en)
- Install development dependencies
  - ```xcode-select --install```
- Install Rust
  - ```curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh```

### Recommended VS Code extensions

- Rust
  - Contains language server and rust-analyzer for error highlighting
- Rust prettier
  - Auto formatting code for better readability (Alt+Shift+F)

## Initializing the app

- Run ```npm create tauri-app@latest``` (note that in Windows this must be run in Powershell, VS Code integrated terminal or Windows terminal, Git bash does not work)
- Select preferred project framework
- Run command ```npm run tauri dev``` to start the example app