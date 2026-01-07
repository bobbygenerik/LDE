# Design Decisions

## Compositor Toolkit
- Choice: Smithay (Rust-native, Wayland-first)
- Reason: better Rust integration, fewer FFI pitfalls, good fit for a Rust-first stack.
- Note: we can still evaluate wlroots bindings later if we hit feature gaps.

## IPC
- Choice: D-Bus with zbus
- Reason: standard Linux desktop IPC, good Rust support.
