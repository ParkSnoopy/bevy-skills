---
name: bevy-voxel-pipeline
description: Use when meshing voxel chunks with `block-mesh-rs` (`greedy_quads` / `visible_block_faces`), translating a `GreedyQuadsBuffer` into a Bevy 0.19 `Mesh` via `try_insert_attribute`, choosing between greedy and simple meshing, or scheduling chunk meshing on `AsyncComputeTaskPool` so the main thread doesn't stall. Covers Bevy 0.19 voxel meshing.
license: MIT
compatibility: opencode,claude-code,cursor
metadata:
  tier: "2"
  area: voxel
  bevy_version: "0.19"
---

# Bevy 0.19 — Voxel meshing pipeline

## When to use this skill

- Building a Minecraft-style voxel world with chunked meshing.
- Choosing between `greedy_quads` (fewer, larger quads — best for static terrain) and `visible_block_faces` (one quad per face — best for fast remesh on edits).
- Wiring `block-mesh-rs` output into a Bevy 0.19 `Mesh`.
- Avoiding main-thread stalls on remesh by spawning the work on `AsyncComputeTaskPool`.

## Canonical pattern

```rust
//! `bevy-voxel-pipeline` skill — chunk meshing threaded off the main thread.
//!
//! Reuses `skill_examples::voxel::{BlockId, mesh_chunk, demo_chunk, ChunkShape}`.
//! `block_mesh 0.2` + `ndshape 0.3` mesh the chunk on `AsyncComputeTaskPool`,
//! a `MeshTask` component polls the resulting `Task<Option<Mesh>>` in `Update`,
//! and `Mesh::try_insert_attribute` (0.18+; returns `Result` — see Rust source
//! for `skill_examples::voxel`) builds the Bevy `Mesh` once the task completes.

use bevy::{
    prelude::*,
    tasks::{
        AsyncComputeTaskPool,
        Task,
    },
};
use futures_lite::future;
use skill_examples::voxel::{
    demo_chunk,
    mesh_chunk,
};

/// Marker for a pending meshing task.
#[derive(Component)]
struct MeshTask(Task<Option<Mesh>>);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, kick_mesh_job))
        .add_systems(Update, poll_mesh_jobs)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(2.0, 4.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Name::new("chunk"),
        Transform::default(),
        Visibility::default(),
    ));
}

fn kick_mesh_job(mut commands: Commands, q: Query<Entity, Added<Name>>) {
    let pool = AsyncComputeTaskPool::get();
    let blocks = demo_chunk();
    for entity in &q {
        let blocks = blocks.clone();
        let task = pool.spawn(async move { mesh_chunk(&blocks) });
        commands.entity(entity).insert(MeshTask(task));
    }
}

fn poll_mesh_jobs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut q: Query<(Entity, &mut MeshTask)>,
) {
    for (entity, mut task) in &mut q {
        if let Some(maybe_mesh) = future::block_on(future::poll_once(&mut task.0)) {
            commands.entity(entity).remove::<MeshTask>();
            if let Some(mesh) = maybe_mesh {
                let handle = meshes.add(mesh);
                commands.entity(entity).insert(Mesh3d(handle));
            }
        }
    }
}
```

## Threading: get it off the main thread

```rust
use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;

#[derive(Component)]
struct MeshTask(Task<Option<Mesh>>);

# fn _kick(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
let pool = AsyncComputeTaskPool::get();
let blocks: Vec<crate::BlockId> = Vec::new(); // load from chunk store
let task = pool.spawn(async move { crate::mesh_chunk(&blocks) });
commands.spawn(MeshTask(task));
# }

# fn _poll(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut q: Query<(Entity, &mut MeshTask)>) {
for (entity, mut task) in &mut q {
    if let Some(maybe_mesh) = future::block_on(future::poll_once(&mut task.0)) {
        commands.entity(entity).remove::<MeshTask>();
        if let Some(mesh) = maybe_mesh {
            let handle = meshes.add(mesh);
            commands.entity(entity).insert(Mesh3d(handle));
        }
    }
}
# }
```

## Gotchas

- **Padding is non-optional.** `block-mesh` needs one cell of padding on each axis so cross-chunk neighbour visibility computes correctly. A "16-cube" chunk is stored as 18×18×18; edits in a chunk also touch the padding of its six neighbours.
- **Greedy vs simple.** `greedy_quads` produces 1/3 the quads of `visible_block_faces` but takes 3× longer. Use simple for chunks players are editing right now, greedy for chunks that have been stable for a while.
- **UVs are not free.** `block-mesh` doesn't emit UVs — you compute them yourself from quad face direction and block ID. A texture atlas + per-quad offset is the standard approach (see `bevy-voxel-data`).
- **`block-mesh-rs` is on Bevy-independent crate 0.2.0** (last updated 2022). It depends on `ilattice` and `ndshape`, both pure-Rust. Compatible with Bevy 0.19 by virtue of not depending on Bevy at all.
- **Don't reach for `par_iter` inside a single chunk mesh** — block-mesh is already fast. The parallelism wins are across chunks, not within one. Spawn N chunk-mesh tasks on `AsyncComputeTaskPool`.
- **`try_insert_attribute` returns `Result<(), MeshAccessError>`** in 0.18 — the error case is "mesh already extracted to render world", which won't happen for a freshly-`new`-d mesh. Still, handle the `Result` so the API doesn't regress on you.
- **`RenderAssetUsages` lives in `bevy::asset`** (re-exported from `bevy::render::render_asset` privately). The public re-export is `bevy::asset::RenderAssetUsages`.

## See also

- `bevy-voxel-data` — RON block definitions, palette, UV atlas baking.
- `bevy-assets` — loading chunk data and managing the resulting Mesh handles.
