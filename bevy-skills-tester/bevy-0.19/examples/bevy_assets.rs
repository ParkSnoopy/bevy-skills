use bevy::{
    asset::AssetPath,
    prelude::*,
    world_serialization::WorldAsset,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes_override: Some(true),
            ..default()
        }))
        .init_resource::<MyHandles>()
        .add_systems(Startup, load_handles)
        .add_systems(Update, react_to_loads)
        .run();
}

#[derive(Resource, Default)]
struct MyHandles {
    hero: Handle<WorldAsset>,
    bricks: Handle<Image>,
}

fn load_handles(asset_server: Res<AssetServer>, mut handles: ResMut<MyHandles>) {
    handles.hero = asset_server.load("models/hero.glb#Scene0");
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

fn asset_paths() {
    let path = AssetPath::from("textures/bricks.png");
    let path_with_label = AssetPath::from("models/hero.glb").with_label("Scene0");
    let _ = path;
    let _ = path_with_label;
}

fn check_ready(asset_server: Res<AssetServer>, handles: Res<MyHandles>) {
    use bevy::asset::LoadState;

    if matches!(asset_server.load_state(&handles.hero), LoadState::Loaded) {
        let _ = &handles.hero;
    }
}
