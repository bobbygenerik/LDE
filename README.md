# Kevyt

Kevyt — Lightweight Desktop Environment (Wayland-first, Rust + GTK4).

## Overview

This repository is an initial scaffold for the Kevyt project, including a Rust
workspace for the compositor, shell, and settings daemon.

## Files

- README.md — this file
- LICENSE — MIT license
- .gitignore — common ignores
- docs/spec.md — MVP spec and IPC outline
- docs/visual-system.md — visual direction
- docs/dbus.md — D-Bus interface draft
- docs/decisions.md — stack decisions
- docs/settings-keys.md — settings keys and defaults
- docs/repo-structure.md — repo layout
- Cargo.toml — Rust workspace
- crates/ — compositor, shell, settings daemon, shared types
- src/main.py — legacy scaffold (to be removed later)

## Quick start

```bash
git clone <your-remote-url> Kevyt
cd Kevyt
```

Rust tools (optional for now):
- `cargo run -p kevyt_shell` (requires GTK4 dev packages)
- `cargo run -p kevyt_compositor`
