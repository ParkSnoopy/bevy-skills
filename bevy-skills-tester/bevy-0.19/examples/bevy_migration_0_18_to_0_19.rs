use bevy::{
    input_focus::{
        FocusCause,
        InputFocus,
    },
    prelude::*,
    world_serialization::{
        DynamicWorld,
        WorldAssetRoot,
    },
};

#[derive(Resource)]
struct Health(u32);

#[derive(Default)]
struct LocalThing;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::new("Migrated text"),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
            font_size: FontSize::Px(24.0),
            ..default()
        },
        TextLayout::justify(Justify::Left),
    ));

    let _family = TextFont {
        font: FontSource::Family("Fira Sans".into()),
        ..default()
    };
    let _linebreak = TextLayout::linebreak(LineBreak::WordBoundary);
    let _no_wrap = TextLayout::no_wrap();
}

fn resource_is_queryable(query: Query<&Health>) {
    for health in &query {
        let _ = health.0;
    }
}

fn focus_api(mut focus: ResMut<InputFocus>, query: Query<Entity, With<Button>>) {
    if let Some(entity) = query.iter().next() {
        focus.set(entity, FocusCause::Navigated);
    }
}

fn world_api(world: &mut World) {
    world.init_non_send::<LocalThing>();
    let _ = world.get_non_send::<LocalThing>();
}

fn serialization_types(_world: Option<DynamicWorld>, _root: Option<WorldAssetRoot>) {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (resource_is_queryable, focus_api))
        .run();
}
