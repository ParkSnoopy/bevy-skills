//! `bevy-custom-assets` skill — an `AssetLoader` for a custom RON level format.
//!
//! In 0.19 the loader struct still must `#[derive(TypePath)]`, and
//! `LoadContext::path()` still returns `AssetPath` (use `.path()` on it for the
//! platform path). `reader.read_to_end` is the basic async reader API.

use bevy::asset::{Asset, AssetApp, AssetLoader, LoadContext, io::Reader};
use bevy::prelude::*;
use bevy::reflect::TypePath;
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
        // Track the referenced texture as a dependency so
        // `AssetEvent::LoadedWithDependencies` fires correctly.
        let _: Handle<Image> = load_context.load(&level.thumbnail);
        let _ = load_context.path().path(); // AssetPath -> platform path
        Ok(level)
    }

    fn extensions(&self) -> &[&str] {
        &["level.ron"]
    }
}

pub struct LevelLoaderPlugin;
impl Plugin for LevelLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<LevelDef>().register_asset_loader(LevelLoader);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LevelLoaderPlugin)
        .add_systems(Startup, request_level)
        .run();
}

#[derive(Resource)]
struct LevelHandle(Handle<LevelDef>);

fn request_level(asset_server: Res<AssetServer>, mut commands: Commands) {
    let handle: Handle<LevelDef> = asset_server.load("levels/example.level.ron");
    commands.insert_resource(LevelHandle(handle));
    commands.spawn(Camera3d::default());
}