use bevy::{
    input::InputSystems,
    prelude::*,
};

fn install_input(app: &mut App) {
    app.init_resource::<ActionBuffer>()
        .add_systems(PreUpdate, translate_physical_input.after(InputSystems));
}

#[derive(Resource, Default)]
struct ActionBuffer;

fn translate_physical_input(mut _actions: ResMut<ActionBuffer>) {}

fn main() {}
