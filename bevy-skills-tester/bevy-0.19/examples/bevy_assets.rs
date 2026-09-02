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
        .add_systems(Update, react_to_loads)
        .run();
}

#[derive(Resource, Default)]
struct MyHandles {
    hero: Handle<WorldAsset>,
    bricks: Handle<Image>,
}

fn load_handles(asset_server: Res<AssetServer>, mut handles: ResMut<MyHandles>) {
    // GLTF scenes are addressed by sub-asset label.
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
