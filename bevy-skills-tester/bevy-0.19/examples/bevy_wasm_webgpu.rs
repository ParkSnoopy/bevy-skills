use bevy::{
    prelude::*,
    render::{
        RenderPlugin,
        settings::{
            Backends,
            WgpuSettings,
        },
    },
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(RenderPlugin {
                render_creation: WgpuSettings {
                    backends: Some(Backends::GL),
                    ..default()
                }
                .into(),
                ..default()
            }),
        )
        .run();
}
