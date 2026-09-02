use bevy::{
    asset::{
        Asset,
        AssetApp,
        AssetLoader,
        LoadContext,
        io::Reader,
    },
    prelude::*,
    reflect::TypePath,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Asset, TypePath, Debug, Deserialize)]
pub struct LevelDef {
    pub name: String,
    pub gravity: f32,
    pub thumbnail: String, // path to a referenced texture asset
}

#[derive(TypePath)] // 0.18: required on the loader itself.
pub struct LevelLoader;

#[derive(Debug, Error)]
pub enum LevelLoaderError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse: {0}")]
    Parse(#[from] ron::error::SpannedError),
}

impl AssetLoader for LevelLoader {
    type Asset = LevelDef;
    type Settings = ();
    type Error = LevelLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        // Pull the whole file. For very large files prefer `seekable()`.
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let level: LevelDef = ron::de::from_bytes(&bytes)?;

        // Pull in a referenced asset so it loads alongside this one.
        // The resulting handle ends up tracked as a dependency.
        let _: Handle<Image> = load_context.load_builder().load(level.thumbnail.clone());

        Ok(level)
    }

    fn extensions(&self) -> &[&str] {
        &["level.ron"]
    }
}

pub struct LevelLoaderPlugin;
impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<LevelDef>()
            .register_asset_loader(LevelLoader);
    }
}

fn main() {}
