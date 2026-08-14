---
name: bevy-cargo-features
description: Use when picking Bevy Cargo features for a project — high-level `2d`/`3d`/`ui`, mid-level `2d_api`/`3d_api`/`ui_api` for custom renderers, opting in to `mouse`/`keyboard`/`gamepad`/`touch`/`gestures` with `default-features = false`, or explicitly adding `ui`/`bevy_audio` no longer implied by `2d`/`3d`. Critical for WASM client size in Bevy 0.19.
license: MIT
compatibility: opencode,claude-code,cursor
metadata:
  tier: "1"
  area: cargo
  bevy_version: "0.19"
---

# Bevy 0.19 — Cargo features

## When to use this skill

- Authoring or auditing a Bevy project's `Cargo.toml`.
- Trimming the WASM bundle (the single biggest lever).
- Building a headless server (no rendering, no windowing, no audio).
- Writing a custom renderer that needs Bevy's data types but not its render pipeline.
- Migrating from a 0.17 manifest — several features were renamed in 0.18.

## Canonical patterns

### High-level: pick one of the three "app shape" collections

```toml
# 3D game (default for most projects)
[dependencies]
bevy = { version = "0.19", features = ["3d"] }

# 2D-only game
[dependencies]
bevy = { version = "0.19", features = ["2d"] }

# UI-only tool (no scene rendering)
[dependencies]
bevy = { version = "0.19", features = ["ui"] }
```

`2d`, `3d`, `ui` are the three top-level **feature collections** new in 0.18. They turn on a curated stack appropriate for that app shape. Use these instead of hand-picking 40 individual flags.

### Mid-level: API without the default renderer

```toml
# I want Bevy's 3D component types and shading hooks,
# but I'm bringing my own render pipeline.
[dependencies]
bevy = { version = "0.19", default-features = false, features = ["3d_api"] }
```

`2d_api`, `3d_api`, `ui_api` enable the **API surface** (component types, asset types, shader types) without enabling the high-level renderer plugins. New in 0.18.

### Headless server

```toml
[dependencies]
# No rendering, no windowing, no audio — just ECS + assets + classic world serialization.
bevy = { version = "0.19", default-features = false, features = [
    "bevy_world_serialization",
    "bevy_asset",
    "serialize",
    "multi_threaded",
] }
```

Combine with `MinimalPlugins` (not `DefaultPlugins`) at runtime.

### Minimal client, opt-in input

```toml
[dependencies]
bevy = { version = "0.19", default-features = false, features = [
    "3d",
    "mouse",
    "keyboard",
    "gamepad",
    # touch, gestures — add only if needed
] }
```

In 0.18, `default-features = false` no longer turns on input by default. Opt in to `mouse`, `keyboard`, `gamepad`, `touch`, `gestures` individually.

## Feature renames (0.17 → 0.18)

| 0.17 name | 0.18 name |
|---|---|
| `animation` | `gltf_animation` |
| `bevy_sprite_picking_backend` | `sprite_picking` |
| `bevy_ui_picking_backend` | `ui_picking` |
| `bevy_mesh_picking_backend` | `mesh_picking` |
| `documentation` | `reflect_documentation` |

CI for a 0.17 → 0.18 upgrade should grep `Cargo.toml` for the left column.

## WASM-specific advice

```toml
# In your client crate's Cargo.toml:
[dependencies]
bevy = { version = "0.19", default-features = false, features = [
    "3d",              # includes window plumbing and WebGL2
    # "webgpu",        # add WebGPU alongside WebGL2 when needed
    "mouse",
    "keyboard",
    "touch",           # mobile browsers
] }

[profile.wasm-release]
inherits = "release"
opt-level = "z"        # size, not speed
lto = "fat"
codegen-units = 1
strip = "debuginfo"
```

Bundle size targets: a stripped `wasm-opt -Oz` release should fit in **5–10 MB** for a moderately-featured 3D client. If your bundle is >20 MB, the most common cause is leaving `default-features = true`.

## Gotchas (0.19)

- **`default-features = true` (or omitted) enables ~30 features.** That's fine for desktop dev, fatal for WASM. Always explicit-feature WASM builds.
- **`2d` and `3d` no longer imply `ui` or audio.** Add `ui`, `bevy_audio`, and the required codec feature explicitly when the app uses them.
- **Dev-vs-release feature split.** A common pattern is `[features] dev = ["bevy/dynamic_linking", "bevy/file_watcher", "bevy/embedded_watcher"]` and only enable `dev` in `cargo run`, not `cargo build --release`.
- **`bevy_dev_tools`** (FPS overlay, system diagnostics) is its own feature — add it in `dev` and gate the plugin behind `#[cfg(feature = "bevy_dev_tools")]`.
- **`multi_threaded`** is a separate Bevy feature in 0.18. On by default through the high-level collections; off in headless minimal builds unless you ask for it.
- **`trace_tracy` / `trace_chrome`** are mutually exclusive — picking both at once will pick one and silently ignore the other.
- **Platform features.** Windows and macOS are selected by the Cargo target, not Bevy features. On Linux, select `x11` and/or `wayland`. For WASM, enable `bevy_winit` and a web backend (`webgl2` or `webgpu`).

## See also

- `bevy-wasm-webgpu` — the WASM-specific story for picking a graphics backend.
- `bevy-migration-0-17-to-0-18` — full feature-rename table and `default-features` change.
