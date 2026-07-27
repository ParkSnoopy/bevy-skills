//! `bevy-assets` skill — AssetServer, `Assets<T>`, `AssetEvent::LoadedWithDependencies`.
//!
//! `AssetEvent<T>` is a `Message` in 0.19: iterate with `MessageReader<AssetEvent<T>>`,
//! not `EventReader`. Hot-reload is dev-only.

use bevy::asset::LoadState;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Hot-reload on file change — dev-only.
            watch_for_changes_override: Some(true),
            ..default()
        }))
        .init_resource::<MyHandles>()
        .add_systems(Startup, load_handles)
        .add_systems(Update, (react_to_loads, check_readiness))
        .run();
}

#[derive(Resource, Default)]
struct MyHandles {
    hero: Handle<Gltf>,
    bricks: Handle<Image>,
}

fn load_handles(asset_server: Res<AssetServer>, mut handles: ResMut<MyHandles>) {
    handles.hero = asset_server.load("models/hero.glb");
    handles.bricks = asset_server.load("textures/bricks.png");
}

fn react_to_loads(mut ev: MessageReader<AssetEvent<Image>>, images: Res<Assets<Image>>) {
    for event in ev.read() {
        if let AssetEvent::LoadedWithDependencies { id } = event {
            if let Some(img) = images.get(*id) {
                info!("image loaded: {}x{}", img.width(), img.height());
            }
        }
    }
}

// `LoadState` is not `PartialEq` in 0.19 — use `matches!` instead of `==`.
fn check_readiness(asset_server: Res<AssetServer>, handles: Res<MyHandles>) {
    if matches!(
        asset_server.load_state(&handles.hero),
        LoadState::Loaded
    ) {
        info!("hero gltf ready");
    }
}