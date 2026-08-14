use bevy::prelude::*;
use bevy_generative::{
    map::{
        Map,
        MapBundle,
        MapPlugin,
    },
    noise::{
        FunctionName,
        Method,
    },
    terrain::{
        Terrain,
        TerrainBundle,
        TerrainPlugin,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MapPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let mut map = Map::default();
    map.size = [512, 512];
    map.image_size = [256, 256];
    map.same_size = false;
    map.anti_aliasing = true;
    map.noise.seed = 42;
    map.noise.method = Method::Perlin;
    map.noise.function.name = Some(FunctionName::Fbm);
    map.noise.function.octaves = 5;

    commands.spawn(MapBundle { map, ..default() });
}

fn add_terrain(app: &mut App) {
    app.add_plugins(TerrainPlugin)
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn((
                Camera3d::default(),
                Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ));
            commands.spawn((PointLight::default(), Transform::from_xyz(-2.0, 2.5, 5.0)));
            commands.spawn(TerrainBundle {
                terrain: Terrain {
                    resolution: 32,
                    height_exponent: 1.5,
                    sea_percent: 10.0,
                    ..default()
                },
                ..default()
            });
        });
}
