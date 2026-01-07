# Kevyt Spec (MVP)
## Near-term Implementation Notes
# Near-term Implementation Notes
# Compositor: Smithay integration for Wayland event loop and output handling.
# Settings: JSON persistence in `~/.config/kevyt/settings.json`.
  - `org.kevyt.Compositor`
  - `org.kevyt.Shell`
  - `org.kevyt.Settings`
# LDE Spec (MVP)

## Goals
- MacOS-adjacent UI language with a distinct, lightweight identity.
- Wayland-first, XWayland fallback.
- Run well on 4 GiB RAM + Intel UHD integrated graphics.
- MVP in 4-6 weeks with a cohesive, usable desktop.

## Non-goals (MVP)
- Full app store.
- Advanced compositing effects (heavy blur, complex shaders).
- Deep power management tuning.

## UX Principles
- Clarity over flash: minimal motion, short durations (150-220ms).
- Calm visual hierarchy with strong typography and spacing.
- Subtle depth (shadows) without heavy blur.
- Consistent input behaviors and simple affordances.

## Core Components
1) Compositor (Wayland)
- Window management, focus, move/resize, workspaces.
- XWayland integration.
- Hotkeys and input device handling.
- Basic animations and damage tracking.
- Implementation target: Smithay (Wayland-first, Rust-native).

2) Shell (GTK4)
- Panel: clock, tray, battery, network, volume.
- Dock: favorites + running apps, auto-hide.
- Launcher: search + app launch.
- Notifications: toast + history.

3) Settings Daemon
- Central settings store and IPC surface.
- Applies theme, animation toggles, input preferences.
- Keys defined in `docs/settings-keys.md`.

## Performance Budgets
- Idle RAM target: < 500 MB total desktop footprint.
- Avoid blur by default; use translucent surfaces sparingly.
- Shell widgets lazy-load and release resources when hidden.

## Module Responsibilities
- `compositor`: Wayland session, surfaces, input, workspaces, window rules.
- `shell`: UI for panel/dock/launcher/notifications.
- `settingsd`: settings storage, validation, and broadcast changes.
- `shared`: shared IPC types and config schema.

## IPC Contracts (Draft)
- Transport: D-Bus.
- Names:
  - `org.lde.Compositor`
  - `org.lde.Shell`
  - `org.lde.Settings`

### Example Methods
- Compositor
  - `FocusWindow(id: u64)`
  - `MoveWindow(id: u64, x: i32, y: i32)`
  - `SetWorkspace(index: u32)`
  - `ListWindows() -> [(id, app_id, title)]`

- Shell
  - `ShowLauncher()`
  - `HideLauncher()`
  - `ShowOverview()`

- Settings
  - `Get(key: string) -> variant`
  - `Set(key: string, value: variant)`
  - `Subscribe(keys: [string])`

## MVP Feature Checklist
- Compositor: basic stacking, workspaces, input, XWayland.
- Shell: panel, dock, launcher, notifications.
- Settings: appearance, display, input.
- Packaging: session file and desktop entry.

## Testing
- Manual smoke tests for login session, app launch, window management.
- Basic integration smoke tests for IPC.

## Near-term Implementation Notes
- Compositor: Smithay integration for Wayland event loop and output handling.
- Settings: JSON persistence in `~/.config/lde/settings.json`.
- Backend: winit for dev, DRM later for production.
- Outputs: sysfs connector discovery fallback when Smithay backend isn't wired yet.
