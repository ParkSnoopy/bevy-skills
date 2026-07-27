//! `bevy` skill — smallest valid Bevy 0.19 app (router snippet).
//!
//! Spawns a `Camera3d` in `Startup`; run with `cargo run --example bevy`.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}
