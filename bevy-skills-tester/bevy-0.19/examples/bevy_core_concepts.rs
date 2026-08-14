use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .add_systems(Startup, spawn_world)
            .add_systems(Update, (read_input, apply_gravity).chain())
            .add_systems(FixedUpdate, simulate_physics)
            .add_systems(PostUpdate, sync_transforms);
    }
}

#[derive(Resource)]
struct Score(u32);

fn spawn_world(mut commands: Commands) {
    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 4.0, 8.0)));
}

fn read_input(_keys: Res<ButtonInput<KeyCode>>) {}
fn apply_gravity(_t: Res<Time>) {}
fn simulate_physics(_t: Res<Time<Fixed>>) {}
fn sync_transforms(_q: Query<&mut Transform>) {}

fn rebuild_index(world: &mut World) {
    let count = world.entities().len();
    world.insert_resource(EntityCount(count));
}

#[derive(Resource)]
struct EntityCount(u32);
