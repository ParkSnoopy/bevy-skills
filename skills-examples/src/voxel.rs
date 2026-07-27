//! Voxel meshing helpers shared by `bevy_voxel_data` and `bevy_voxel_pipeline`.
//!
//! Mirrors the `bevy-voxel-pipeline` skill: `block_mesh 0.2` + `ndshape 0.3`
//! glued into a Bevy 0.19 `Mesh`.

use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use block_mesh::ndshape::{ConstShape, ConstShape3u32};
use block_mesh::{
    greedy_quads, GreedyQuadsBuffer, MergeVoxel, Voxel, VoxelVisibility,
    RIGHT_HANDED_Y_UP_CONFIG,
};

/// 18^3: a 16^3 chunk with one cell of padding on each axis so
/// neighbour lookups stay in-bounds. Matches the skill's convention.
pub type ChunkShape = ConstShape3u32<18, 18, 18>;

#[derive(Clone, Copy, Eq, PartialEq, Default, Debug)]
pub struct BlockId(pub u16);

impl Voxel for BlockId {
    fn get_visibility(&self) -> VoxelVisibility {
        if self.0 == 0 {
            VoxelVisibility::Empty
        } else {
            VoxelVisibility::Opaque
        }
    }
}

impl MergeVoxel for BlockId {
    type MergeValue = u16;
    fn merge_value(&self) -> Self::MergeValue {
        self.0
    }
}

/// Build a Bevy 0.19 `Mesh` from a padded chunk of `BlockId`s.
/// Returns `None` for an entirely empty chunk so callers can skip spawning.
pub fn mesh_chunk(blocks: &[BlockId]) -> Option<Mesh> {
    assert_eq!(blocks.len(), ChunkShape::SIZE as usize);

    let mut buffer = GreedyQuadsBuffer::new(blocks.len());
    greedy_quads(
        blocks,
        &ChunkShape {},
        [0; 3],
        [17, 17, 17],
        &RIGHT_HANDED_Y_UP_CONFIG.faces,
        &mut buffer,
    );

    if buffer.quads.num_quads() == 0 {
        return None;
    }

    let num_indices = buffer.quads.num_quads() * 6;
    let num_vertices = buffer.quads.num_quads() * 4;
    let mut indices = Vec::with_capacity(num_indices);
    let mut positions = Vec::with_capacity(num_vertices);
    let mut normals = Vec::with_capacity(num_vertices);

    for (group, face) in buffer
        .quads
        .groups
        .iter()
        .zip(RIGHT_HANDED_Y_UP_CONFIG.faces.iter())
    {
        for quad in group.iter() {
            indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
            positions.extend_from_slice(&face.quad_mesh_positions(quad, 1.0));
            normals.extend_from_slice(&face.quad_mesh_normals());
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.try_insert_attribute(Mesh::ATTRIBUTE_POSITION, positions).ok()?;
    mesh.try_insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals).ok()?;
    mesh.insert_indices(Indices::U32(indices));
    Some(mesh)
}

/// Build a flat 18^3 chunk with a small `1x1x1` stone cube at the centre
/// so voxel examples have visible geometry without a world generator.
pub fn demo_chunk() -> Vec<BlockId> {
    let mut blocks = vec![BlockId(0); ChunkShape::SIZE as usize];
    // Offset 1 for padding; centre of a 16-cube is at (8,8,8).
    let [x, y, z] = [1u32 + 8, 1u32 + 8, 1u32 + 8];
    let idx = ChunkShape::linearize([x, y, z]);
    blocks[idx as usize] = BlockId(1);
    blocks
}