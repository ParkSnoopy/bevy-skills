---
name: bevy-generative
description: Use when generating a procedural map with `MapPlugin` / `MapBundle`, a mesh with `TerrainPlugin` / `TerrainBundle` or `PlanetPlugin` / `PlanetBundle`, configuring `Noise`, `Method`, `FunctionName`, `Region`, or exporting generated assets through `bevy_generative` tag `b0.19.0` in Bevy 0.19.
license: MIT
compatibility: opencode,claude-code,cursor
metadata:
  tier: "4"
  area: procedural-generation
  bevy_version: "0.19"
  target_version: "Bevy 0.19"
---

# Bevy 0.19 — `bevy_generative` procedural maps, terrain, and planets

`bevy_generative` provides Bevy components plus plugins that regenerate a 2D map image, a terrain mesh, or a cube-sphere planet mesh from noise configuration. This skill is specifically verified against the `b0.19.0` tag in the ParkSnoopy fork—not the crate release or the repository's default branch.

## When to use this skill

- Rendering a noise-coloured 2D UI map with `MapPlugin` and `MapBundle`.
- Generating a `Mesh3d` terrain with `TerrainPlugin` / `TerrainBundle`.
- Generating a noise-displaced cube-sphere with `PlanetPlugin` / `PlanetBundle`.
- Selecting a noise `Method` (`Perlin`, `OpenSimplex`, `Worley`, etc.) and a fractal `FunctionName` such as `Fbm` or `RidgedMulti`.
- Configuring a seed, scale, regions, gradient, resolution, wireframe mode, or an export action.

Do **not** use it in a Bevy 0.18 application: this tag depends on Bevy 0.19. Migrate the application first with `bevy-migration-0-18-to-0-19`.

## Git dependency: pin the requested tag

```toml
[dependencies]
bevy = "0.19"
bevy_generative = { git = "https://github.com/ParkSnoopy/bevy_generative", tag = "b0.19.0" }
```

Do not replace `tag` with a floating branch or an unverified crates.io version. The tag fixes the external API this skill describes.

## Canonical pattern: a configured UI map

```rust
use bevy::prelude::*;
use bevy_generative::{
    map::{Map, MapBundle, MapPlugin},
    noise::{FunctionName, Method},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MapPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // `Noise::size` is private, so configure the default owned by `Map`.
    let mut map = Map::default();
    map.size = [512, 512];
    map.image_size = [256, 256];
    map.same_size = false;
    map.anti_aliasing = true;
    map.noise.seed = 42;
    map.noise.method = Method::Perlin;
    map.noise.function.name = Some(FunctionName::Fbm);
    map.noise.function.octaves = 5;

    commands.spawn(MapBundle { map, ..default() });
}
```

The map plugin writes the generated `Image` to `MapBundle`'s `ImageNode`; it is therefore a UI entity and needs a `Camera2d`. Keep `same_size: true` when the texture must preserve the noise-map dimensions. Set it to `false` only to resize to `image_size`, choosing triangle filtering with `anti_aliasing: true` or nearest-neighbour filtering with `false`.

## Select the generated asset

| Asset | Add this plugin | Spawn this bundle | Camera and light |
|---|---|---|---|
| UI map / texture | `MapPlugin` | `MapBundle` | `Camera2d` |
| Heightfield terrain | `TerrainPlugin` | `TerrainBundle` | `Camera3d` plus light |
| Cube-sphere planet | `PlanetPlugin` | `PlanetBundle` | `Camera3d` plus light |

```rust
use bevy::prelude::*;
use bevy_generative::terrain::{Terrain, TerrainBundle, TerrainPlugin};

fn add_terrain(app: &mut App) {
    app.add_plugins(TerrainPlugin)
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn((
                Camera3d::default(),
                Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ));
            commands.spawn((PointLight::default(), Transform::from_xyz(-2.0, 2.5, 5.0)));
            commands.spawn(TerrainBundle {
                terrain: Terrain {
                    resolution: 32,
                    height_exponent: 1.5,
                    sea_percent: 10.0,
                    ..default()
                },
                ..default()
            });
        });
}
```

`TerrainBundle` and `PlanetBundle` contain a generated `(Mesh3d, MeshMaterial3d<StandardMaterial>)`; configure their `Terrain` or `Planet` fields rather than replacing those generated handles.

## Noise and colour configuration

- Use `seed` for deterministic regeneration, `scale` for feature size, and `offset` to pan the sampled noise field.
- `Method` selects the base algorithm: `Perlin`, `OpenSimplex`, `PerlinSurflet`, `Simplex`, `SuperSimplex`, `Value`, or `Worley`.
- `Function { name: Some(FunctionName::Fbm), octaves, frequency, lacunarity, persistence }` layers fractal noise. Set `name: None` for the base method without fractal layering.
- `regions: Vec<Region>` map noise values (0–100 for maps and terrain) to RGBA colours. Define monotonic `Region::position` values; the plugin derives a `colorgrad` domain from them.
- For planets, `resolution` is per cube face and grows mesh cost rapidly. Start low, then raise it after establishing a frame-time budget.

## Gotchas

- **Every generator runs in `Update`, not only when values change.** At `b0.19.0`, `MapPlugin`, `TerrainPlugin`, and `PlanetPlugin` regenerate each matching entity every frame. Do not use large dimensions or high resolution for static content without profiling; use a small generation entity or fork/gate the plugin if you need change-driven generation.
- **Map output is UI, not a world sprite.** `MapBundle` has an `ImageNode`; spawn `Camera2d`, and do not expect `Sprite` or `Mesh2d` components.
- **`size` and `resolution` multiply terrain work.** Terrain sets its internal noise dimensions to `size * resolution`; reducing just one can still leave expensive generation.
- **`export: true` is an action flag.** The generator exports once and resets it to `false`. Native export opens an `rfd` save dialog; WASM export creates a browser download. Trigger it from explicit UI input, not on startup.
- **A git dependency may duplicate Bevy.** Keep the app's `bevy = "0.19"` compatible with the tag. Cargo resolving a second Bevy version makes Bevy types from the two dependency graphs incompatible.
- **This repository otherwise targets Bevy 0.18.** Do not copy this skill's 0.19 code into an 0.18 application; first complete the migration and validate all third-party dependencies.

## See also

- [`bevy-migration-0-18-to-0-19`](../bevy-migration-0-18-to-0-19/SKILL.md) — required migration guide for this 0.19-only dependency.
- [`bevy-assets`](../bevy-assets/SKILL.md) — generated `Image` and `Mesh` assets, handles, and asset lifecycle.
- [`bevy-pbr-materials`](../bevy-pbr-materials/SKILL.md) — `StandardMaterial` and generated terrain/planet mesh presentation.
- [`bevy-cameras`](../bevy-cameras/SKILL.md) — framing the 2D and 3D outputs.
