//! `bevy-voxel-pipeline` skill — chunk meshing threaded off the main thread.
//!
//! Reuses `skill_examples::voxel::{BlockId, mesh_chunk, demo_chunk, ChunkShape}`.
//! `block_mesh 0.2` + `ndshape 0.3` mesh the chunk on `AsyncComputeTaskPool`,
//! a `MeshTask` component polls the resulting `Task<Option<Mesh>>` in `Update`,
//! and `Mesh::try_insert_attribute` (0.18+; returns `Result` — see Rust source
//! for `skill_examples::voxel`) builds the Bevy `Mesh` once the task completes.

use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use futures_lite::future;
use skill_examples::voxel::{demo_chunk, mesh_chunk};

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