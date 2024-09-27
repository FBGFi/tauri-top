# Calling backend with commands

[Documentation](https://tauri.app/v1/guides/features/command/)

Tauri provides api for calling the backend via commands. Commands must be added to the Tauri context when backend is started, and they can be called in two ways:

```js
// When using the Tauri API npm package:
import { invoke } from '@tauri-apps/api/tauri'
// When using the Tauri global script (if not using the npm package)
// Be sure to set `build.withGlobalTauri` in `tauri.conf.json` to true
const invoke = window.__TAURI__.invoke

// Invoke the command
invoke('my_custom_command')
```

Commands can return a value when they are awaited and they can be passed a list of arguments in the invoke call (note that arguments must be defined in snake_case in the backend, and in camelCase in the frontend).