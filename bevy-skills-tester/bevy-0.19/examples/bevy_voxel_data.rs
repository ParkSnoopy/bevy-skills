use std::collections::HashMap;

use bevy::{
    asset::Asset,
    prelude::Resource,
    reflect::TypePath,
};
use serde::Deserialize;

pub type BlockId = u16;

#[derive(Debug, Deserialize, Clone, Eq, Hash, PartialEq)]
#[serde(transparent)]
pub struct StableBlockId(pub String);

#[derive(Debug, Deserialize, Asset, TypePath)]
pub struct BlockCatalog {
    pub blocks: Vec<BlockDef>,
}

#[derive(Debug, Deserialize)]
pub struct BlockDef {
    pub id: StableBlockId,
    pub name: String,
    pub visibility: Visibility,
}

#[derive(Resource, Default)]
pub struct Palette {
    pub by_id: Vec<PaletteEntry>, // dense, process-local BlockId
    pub by_stable_id: HashMap<StableBlockId, BlockId>,
}

#[derive(Clone)]
pub struct PaletteEntry {
    pub stable_id: StableBlockId,
    pub name: String,
    pub visibility: Visibility,
    /// Atlas tile per face: [−X, −Y, −Z, +X, +Y, +Z] (block-mesh order).
    pub face_tiles: [u16; 6],
}

#[derive(Clone, Debug, Deserialize)]
pub enum Visibility {
    Empty,
    Translucent,
    Opaque,
}

fn main() {}
