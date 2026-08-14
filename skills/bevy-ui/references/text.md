# Bevy 0.19 UI — Text

Bevy 0.19 uses a **split-component model** for text. There is no `TextBundle`.
Instead, a text entity is a bundle of separate components, each with a single
responsibility.

## Quick reference

| Component | Purpose |
|---|---|
| `Text::new("hello")` | The string content of a text entity. |
| `TextFont { font, font_size, .. }` | `FontSource` + `FontSize`. |
| `TextColor(Color)` | Foreground colour. |
| `TextShadow::default()` | Adds a subtle drop shadow. |
| `TextLayout` | Wrapping and alignment; optional. |

`Text`, `TextFont`, `TextColor`, and `TextShadow` are all in `bevy::prelude`.

## Common patterns

### Inline text child (the canonical pattern)

```rust
use bevy::prelude::*;

fn text_label(asset_server: &AssetServer) -> impl Bundle {
    (
        Text::new("Hello, Bevy!"),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
            font_size: FontSize::Px(24.0),
            ..default()
        },
        TextColor(Color::WHITE),
        TextShadow::default(),
    )
}
```

### Text with layout control

```rust
use bevy::prelude::*;
use bevy::text::{Justify, TextLayout};

fn wrapped_text(asset_server: &AssetServer) -> impl Bundle {
    (
        Text::new("A longer paragraph that wraps across lines."),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
            font_size: FontSize::Px(18.0),
            ..default()
        },
        TextColor(Color::srgb(0.8, 0.8, 0.8)),
        TextLayout::justify(Justify::Left),
    )
}
```

Note: `TextLayout::new_with_justify` is stale in 0.19; use `TextLayout::justify`.

### Choosing an installed font family

Bevy 0.19 uses `FontSource` and Parley font fallback. Select an installed family
directly when the `system_font_discovery` Cargo feature is enabled:

```rust
use bevy::prelude::*;

fn text_with_system_font() -> impl Bundle {
    (
        Text::new("Hello"),
        TextFont {
            font: FontSource::Family("Fira Sans".into()),
            font_size: FontSize::Px(24.0),
            ..default()
        },
    )
}
```

For bundled fonts, continue loading the asset and convert the handle with `.into()`.
The old `TextFont::default().font` handle-replacement trick is obsolete.

## Pitfalls

- **No `TextBundle` in 0.19.** Pre-0.17 tutorials use `TextBundle { style, text, .. }`.
  This type does not exist in 0.19. Spawn the components individually or as a tuple bundle.

- **`Text` deref.** `Text` derefs to `String` via `Deref` / `DerefMut`. To update
  the text in a system: `**text = "New string".to_string();`.

- **Font size units are explicit.** Use `FontSize::Px`, `FontSize::Vh`, or
  `FontSize::Rem`; a bare `f32` no longer compiles.

- **`TextShadow` is a unit-like default.** `TextShadow::default()` gives a `(1.0, 1.0)`
  pixel shadow in dark grey. To customise, construct `TextShadow { offset: Vec2::new(2.0, 2.0), color: Color::BLACK }`.
