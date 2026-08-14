---
name: bevy-migration-0-18-to-0-19
description: Use when upgrading from Bevy 0.18 to Bevy 0.19, when `TextLayout::new_with_justify` or `TextFont { font_size: 24.0, font: handle }` stops compiling (now `TextLayout::justify`, `FontSize::Px`, `FontSource`), when `bevy_scene`/`DynamicScene` serialization moved to `bevy_world_serialization` (`WorldAsset`/`DynamicWorld`), when a `#[derive(Resource)]` type now also matches `&T` component queries, or when `init_non_send_resource`/`FeathersPlugin`/`EasyScreenRecordPlugin` changed. Index of every breaking Bevy 0.19 change.
license: MIT
compatibility: opencode,claude-code,cursor
metadata:
  tier: "1"
  area: migration
  bevy_version: "0.19"
  target_version: "Bevy 0.19"
---

# Bevy 0.18 → 0.19 — Migration cheat sheet

**Released 2026-06-18.** Apply top-down — earlier items break later builds if skipped.

## When to use this skill

- Compiler errors after a `bevy = "0.18"` → `bevy = "0.19"` bump.
- LLM emits 0.18-era text/font/scene API names from training data.
- A `#[derive(Resource)]` type suddenly behaves like a component.
- A third-party crate has not shipped a Bevy 0.19-compatible release yet.

## The renames you'll hit first

### Text layout: drop the `new_with_` prefix

```rust
// 0.19
TextLayout::justify(Justify::Left)   // was new_with_justify
TextLayout::linebreak(LineBreak::WordBoundary) // was new_with_linebreak
TextLayout::no_wrap()                // was new_with_no_wrap
```

### `TextFont` fields changed type: `FontSource` + `FontSize`

```rust
// 0.19 — font is FontSource (handle OR family), font_size is FontSize
TextFont {
    font: asset_server.load("fonts/FiraSans-Bold.ttf").into(), // Handle → FontSource via .into()
    font_size: FontSize::Px(24.0),                             // f32 → FontSize::Px / ::Vh / ::Rem
    ..default()
}
// Select by family (needs the `bevy/system_font_discovery` feature for installed fonts):
TextFont {
    font: FontSource::Family("Fira Sans".into()),
    ..default()
}
```

0.19 swapped the cosmic-text layout backend for **Parley**, which does font fallback
automatically via `fontique`. The old "insert a `DefaultFontHandle` so `..default()`
entities inherit a font" trick is obsolete — see [references/text-and-fonts.md](references/text-and-fonts.md).

### Scene serialization crate was renamed (silent wrong-crate trap)

`bevy_scene` is **reused** for the new BSN system. Classic `DynamicScene` round-trip
serialization moved to **`bevy_world_serialization`**:

```rust
// 0.18  → 0.19
// bevy_scene::DynamicScene          → bevy_world_serialization::DynamicWorld
// bevy::scene::SceneRoot            → bevy::world_serialization::WorldAssetRoot
// DynamicSceneBuilder               → DynamicWorldBuilder
// SceneSpawner                      → WorldInstanceSpawner
```

glTF scene spawning still uses the old (now `world_serialization`) system. Full table:
[references/scene-serialization.md](references/scene-serialization.md).

### Resources are now Components (mostly a mental-model change)

`Res<T>`, `ResMut<T>`, `#[derive(Resource)]`, and `insert_resource` **all still work.**
The narrow breakages:

```rust
// ILLEGAL in 0.19 — a type can no longer be both:
#[derive(Component, Resource)] struct Health(u32);
```

Plus: a resource type now also matches `Query<&T>`, and inserting it as a *component*
can despawn other copies. Non-send resource methods were renamed (deprecated, not removed):

```rust
world.init_non_send::<T>();    // was init_non_send_resource
world.get_non_send::<T>();     // was get_non_send_resource
```

Details + the query/despawn footgun: [references/resources-as-components.md](references/resources-as-components.md).

## Cargo feature implications tightened

```toml
# 0.19: audio is NO LONGER implied by 2d / 3d / ui — add it explicitly:
bevy = { version = "0.19", features = ["3d", "bevy_audio", "vorbis"] }
# ui is NO LONGER implied by 2d / 3d either — add "ui" if you build UI.
```

## Feathers UI stabilized

`experimental_` dropped. `FeathersPlugin` → `FeathersCorePlugin`, and widget components
lost their `Core` prefix (`CoreScrollbarThumb` → `ScrollbarThumb`,
`CoreSliderDragState` → `SliderDragState`, etc.). Only relevant if you use Feathers.

## Third-party crates lag the release

Check every Bevy-coupled package before migrating it. `bevy_capture 0.6.0`,
`es-fluent-manager-bevy 0.19.2`, `bevy_hanabi 0.19.0`,
`bevy_spritesheet_animation 7.0.1`, and
`bevy_vector_shapes 0.13.1` support Bevy 0.19. The `bevy-vfx` skill targets those
versions while marking `bevy_spark 0.2.0` as an explicit Bevy 0.18-only exception;
do not combine `bevy_spark` with a Bevy 0.19 dependency graph.

## Gotchas

- **`#[reflect(Resource)]`** reflection access now also needs `ReflectComponent` in some paths.
- **`TextFont::default().font` is a `FontSource`, not a `Handle<Font>`** — code that read it as a handle won't compile.
- **System font discovery** needs the `bevy/system_font_discovery` feature; on Linux also `libfontconfig1-dev`.
- The fallback setters (`set_serif_family`, etc.) now return `Result` — and you mostly shouldn't need them under Parley.

## See also

- `bevy-migration-0-17-to-0-18` — the previous release's rename catalogue.
- `bevy-ui` — `TextFont`/`FontSize`/`FontSource` in everyday UI code.
- `bevy-cargo-features` — feature-implication table and renames.
- `bevy-ecs-components` — the Component model the new Resource subtrait plugs into.
