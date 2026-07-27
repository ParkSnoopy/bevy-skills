//! Shared types reused across `skill-examples` examples.
//!
//! Each example targets a specific `bevy-skills` skill and must compile
//! against `bevy = "0.19"`. See the matching `skills/<name>/SKILL.md` for
//! the prose these examples exercise.

pub mod voxel;

pub use voxel::{BlockId, ChunkShape, mesh_chunk};