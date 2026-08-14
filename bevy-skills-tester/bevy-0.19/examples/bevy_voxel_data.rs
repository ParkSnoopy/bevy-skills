use std::collections::HashMap;

use bevy::{
    asset::Asset,
    prelude::Resource,
    reflect::TypePath,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Asset, TypePath)]
pub struct BlockCatalog {
    pub blocks: Vec<BlockDef>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct BlockFaces {
    #[serde(default)]
    pub top: Option<String>,
    #[serde(default)]
    pub bottom: Option<String>,
    #[serde(default)]
    pub side: Option<String>,
    #[serde(default)]
    pub all: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BlockDef {
    pub name: String,
    pub visibility: Visibility,
    #[serde(default)]
    pub faces: Option<BlockFaces>,
    #[serde(default)]
    pub flags: Vec<String>,
}

#[derive(Resource, Default)]
pub struct Palette {
    pub by_id: Vec<PaletteEntry>,
    pub by_name: HashMap<String, u16>,
}

#[derive(Default, Clone)]
pub struct PaletteEntry {
    pub name: String,
    pub visibility: Visibility,
    pub face_tiles: [u16; 6],
}

#[derive(Debug, Deserialize, Clone, Copy, Default)]
pub enum Visibility {
    #[default]
    Empty,
    Translucent,
    Opaque,
}

fn main() {}
