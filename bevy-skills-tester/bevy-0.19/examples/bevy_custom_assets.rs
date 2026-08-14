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
    pub thumbnail: String,
}

#[derive(TypePath)]
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
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let level: LevelDef = ron::de::from_bytes(&bytes)?;
        let _: Handle<Image> = load_context.load(&level.thumbnail);
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

async fn seekable_reader(reader: &mut dyn Reader) -> std::io::Result<()> {
    match reader.seekable() {
        Ok(seekable) => {
            let _ = seekable;
        }
        Err(_) => {}
    }
    Ok(())
}

fn main() {
    App::new().add_plugins(LevelLoaderPlugin).run();
}
