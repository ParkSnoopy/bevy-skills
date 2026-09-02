use bevy::{
    asset::RenderAssetUsages,
    mesh::{
        Indices,
        PrimitiveTopology,
    },
    prelude::*,
};
use block_mesh::{
    GreedyQuadsBuffer,
    MergeVoxel,
    RIGHT_HANDED_Y_UP_CONFIG,
    Voxel,
    VoxelVisibility,
    greedy_quads,
    ndshape::{
        ConstShape,
        ConstShape3u32,
    },
};

// 18^3 — the standard "chunk plus padding" block-mesh expects.
// Each axis gets 1 cell of padding on each side so neighbour lookups are
// in-bounds. The user-visible chunk is 16^3.
type ChunkShape = ConstShape3u32<18, 18, 18>;

#[derive(Clone, Copy, Eq, PartialEq, Default)]
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

/// Build a Bevy 0.19 Mesh from a padded chunk of blocks.
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

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.try_insert_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .ok()?;
    mesh.try_insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .ok()?;
    mesh.insert_indices(Indices::U32(indices));
    Some(mesh)
}

fn main() {}
