//! `bevy-voxel-data` skill — RON `BlockCatalog` asset + runtime `Palette` resource.
//!
//! Defines `BlockCatalog` as a Bevy `Asset` (deserialised from RON), then
//! shows the `Palette` `Resource` that the meshing pass (see `bevy_voxel_pipeline`)
//! looks up tiles from. Doesn't load an actual file — constructs the catalog
//! in code via `AssetServer::load` against a path you'd ship at
//! `assets/blocks.ron` in a real project.

use bevy::asset::{Asset, AssetApp, AssetLoader, LoadContext, io::Reader};
use bevy::prelude::*;
use bevy::reflect::TypePath;
use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

/// One entry in the on-disk RON block catalog.
#[derive(Debug, Clone, Deserialize)]
pub struct BlockDef {
    pub name: String,
    pub visibility: BlockVisibility,
    pub faces: BlockFaces,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub enum BlockVisibility {
    #[default]
    Empty,
    Opaque,
    Translucent,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct BlockFaces {
    pub all: Option<String>,
    pub top: Option<String>,
    pub bottom: Option<String>,
    pub side: Option<String>,
}

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct BlockCatalog {
    pub blocks: Vec<BlockDef>,
}

#[derive(TypePath)]
pub struct CatalogLoader;

#[derive(Debug, Error)]
pub enum CatalogLoaderError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse: {0}")]
    Parse(#[from] ron::error::SpannedError),
}

impl AssetLoader for CatalogLoader {
    type Asset = BlockCatalog;
    type Settings = ();
    type Error = CatalogLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let catalog: BlockCatalog = ron::de::from_bytes(&bytes)?;
        Ok(catalog)
    }

    fn extensions(&self) -> &[&str] {
        &["blocks.ron"]
    }
}

pub struct CatalogLoaderPlugin;
impl Plugin for CatalogLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<BlockCatalog>()
            .register_asset_loader(CatalogLoader);
    }
}

/// Runtime palette keyed by `BlockId` (u16). Index-order == catalog order.
/// A real game populates `face_tiles` from the KTX2 atlas JSON sidecar;
/// here we leave it zeroed so the pipeline example doesn't need the atlas.
#[derive(Resource, Default)]
pub struct Palette {
    pub by_id: Vec<PaletteEntry>,
    pub by_name: HashMap<String, u16>,
}

#[derive(Default, Clone)]
pub struct PaletteEntry {
    pub name: String,
    pub visibility: BlockVisibility,
    /// Atlas tile per face: [−X, −Y, −Z, +X, +Y, +Z] (block-mesh order).
    pub face_tiles: [u16; 6],
}

/// Build a `Palette` from a loaded `BlockCatalog`.
pub fn build_palette(catalog: &BlockCatalog) -> Palette {
    let mut palette = Palette::default();
    for (id, block) in catalog.blocks.iter().enumerate() {
        let id_u16 = id as u16;
        palette.by_id.push(PaletteEntry {
            name: block.name.clone(),
            visibility: block.visibility.clone(),
            face_tiles: [0; 6],
        });
        palette.by_name.insert(block.name.clone(), id_u16);
    }
    palette
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CatalogLoaderPlugin)
        .add_systems(Startup, request_catalog)
        .add_systems(Update, populate_palette)
        .run();
}

#[derive(Resource)]
struct CatalogHandle(Handle<BlockCatalog>);

fn request_catalog(asset_server: Res<AssetServer>, mut commands: Commands) {
    let handle: Handle<BlockCatalog> = asset_server.load("blocks.ron");
    commands.insert_resource(CatalogHandle(handle));
    commands.spawn(Camera3d::default());
}

fn populate_palette(
    catalog: Res<Assets<BlockCatalog>>,
    handle: Res<CatalogHandle>,
    mut commands: Commands,
) {
    if let Some(catalog) = catalog.get(&handle.0) {
        let palette = build_palette(catalog);
        info!("palette built with {} blocks", palette.by_id.len());
        commands.insert_resource(palette);
        commands.remove_resource::<CatalogHandle>();
    }
}