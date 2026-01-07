# Kevyt Visual System (Draft)

## Tone
- Sleek, light, calm. Distinct from macOS by typography and color.
- Minimal translucency and soft, narrow shadows.

## Color Palette
- Base: warm gray backgrounds with cool neutral surfaces.
- Accent: muted blue-green (no neon).
- Status: red, amber, green for warnings.

Example tokens:
- `base-0`: #f6f5f2
- `base-1`: #efeeea
- `surface-0`: #ffffff
- `surface-1`: #f9f8f6
- `accent-0`: #3e7f7a
- `text-0`: #1f2328
- `text-1`: #5a626b
- `shadow-0`: rgba(0, 0, 0, 0.12)

## Typography
- Primary: "IBM Plex Sans" or "Source Sans 3".
- Secondary (mono, settings): "IBM Plex Mono".
- Titles: 14-16px; body: 12-13px; UI labels: 11-12px.

## Spacing + Radius
- 4px spacing grid.
- Corners: 8px for panels, 10px for dialogs, 12px for dock tiles.

## Motion
- 150-220ms ease-out for open/close.
- 80-120ms for hover emphasis.
- Reduced motion setting disables non-essential animations.

## Iconography
- Rounded rect silhouette, thin strokes.
- Monochrome for system UI; accent only for primary actions.

## Visual Rules
- Avoid full-window blur; use subtle translucency if needed.
- Shadows tight and soft to avoid GPU heavy rendering.
- Maintain 8-12px padding in panels/dock.
