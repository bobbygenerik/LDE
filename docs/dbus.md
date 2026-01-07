# D-Bus Interfaces (Draft)

## org.kevyt.Settings
- Object path: `/org/kevyt/Settings`

Methods:
- `get(key: s) -> v`
- `set(key: s, value: v)`
- `list() -> a{sv}`

Signals:
- `changed(key: s, value: v)`

## org.kevyt.Compositor
- Object path: `/org/kevyt/Compositor`

Methods:
- `focus_window(id: t)`
- `move_window(id: t, x: i, y: i)`
- `set_workspace(index: u)`
- `list_windows() -> a(ts)`

## org.kevyt.Shell
- Object path: `/org/kevyt/Shell`

Methods:
- `show_launcher()`
- `hide_launcher()`
- `show_overview()`
