# Tauri Plugins

[Documentation](https://tauri.app/v1/guides/features/plugin)

In previous example, we created our own logic for handling low level operations. Essentially this has no difference to what plugins are. Plugins are just collection of code that somebody has created, which can be applied to the backend and then be called from the frontend.

In this example we add plugin for creating a persistent key-value store using [tauri-plugin-store](https://github.com/tauri-apps/tauri-plugin-store), which we use to save theming options for the application.

