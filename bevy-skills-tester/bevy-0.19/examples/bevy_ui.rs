use bevy::{
    input_focus::{
        FocusCause,
        InputFocus,
    },
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // includes InputFocusPlugin in 0.19
        .add_systems(Startup, setup)
        .add_systems(Update, style_button)
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Button,
            AccessibleLabel::new("Start game"),
            Node {
                width: px(180),
                height: px(64),
                border: UiRect::all(px(3)),
                border_radius: BorderRadius::MAX,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.12, 0.12, 0.15)),
            BorderColor::all(Color::WHITE),
            children![(
                Text::new("Start game"),
                TextFont {
                    font: assets.load("fonts/FiraSans-Bold.ttf").into(),
                    font_size: FontSize::Px(32.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            )],
        )],
    ));
}

fn style_button(
    mut focus: ResMut<InputFocus>,
    mut buttons: Query<
        (Entity, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (entity, interaction, mut background) in &mut buttons {
        *background = match interaction {
            Interaction::Pressed => {
                focus.set(entity, FocusCause::Pressed);
                BackgroundColor(Color::srgb(0.15, 0.55, 0.25))
            }
            Interaction::Hovered => BackgroundColor(Color::srgb(0.22, 0.22, 0.28)),
            Interaction::None => BackgroundColor(Color::srgb(0.12, 0.12, 0.15)),
        };
    }
}
