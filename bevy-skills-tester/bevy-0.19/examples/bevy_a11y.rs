use bevy::{
    input_focus::{
        directional_navigation::DirectionalNavigationPlugin,
        tab_navigation::TabNavigationPlugin,
    },
    prelude::*,
    ui::auto_directional_navigation::AutoDirectionalNavigation,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TabNavigationPlugin,
            DirectionalNavigationPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Button,
        AccessibleLabel::new("Start game"),
        AutoDirectionalNavigation::default(),
        children![Text::new("Start game")],
    ));
}
