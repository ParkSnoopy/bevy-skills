//! `bevy-ui` skill — `Node`, `Button`, `TextFont` (0.19 `FontSource` + `FontSize::Px`).
//!
//! 0.19 text backend swap (cosmic-text -> Parley/fontique) made the following
//! changes vs 0.18:
//!   * `TextFont::font` is `FontSource`, not `Handle<Font>`. `From<Handle<Font>>`
//!     is implemented, so `.into()` adapts the load call.
//!   * `TextFont::font_size` is `FontSize::Px(24.0)`, not a bare `f32`.
//!   * `TextLayout::justify(j)` / `::linebreak(..)` / `::no_wrap()` replace
//!     `new_with_justify` / `new_with_linebreak` / `new_with_no_wrap`.
//!
//! Frame-0 `Changed<Interaction>` invariant still applies: the spawn-time
//! `BorderColor` is what shows at frame 0 (the `Interaction::None` arm only
//! runs after the first mouse event), so set it to the colour you want at
//! spawn to avoid a frame-0 mismatch.

use bevy::input_focus::{FocusCause, InputFocus};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<InputFocus>()
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        // Full-screen flex container — centres the button.
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Button,
            Node {
                width: px(150),
                height: px(65),
                border: UiRect::all(px(5)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::MAX,
                ..default()
            },
            // Spawn-time border is what shows at frame 0 — `Changed<Interaction>`
            // does NOT fire on startup, so set the colour you want at spawn.
            BorderColor::all(Color::BLACK),
            BackgroundColor(Color::BLACK),
            children![(
                Text::new("Button"),
                TextFont {
                    font: asset_server
                        .load("fonts/FiraSans-Bold.ttf")
                        .into(), // Handle<Font> -> FontSource via From
                    font_size: FontSize::Px(33.0), // 0.19: FontSize, not bare f32
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default(),
            )]
        )],
    ));
}

fn button_system(
    mut input_focus: ResMut<InputFocus>,
    mut query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Button,
        ),
        Changed<Interaction>,
    >,
) {
    for (entity, interaction, mut bg, mut border, mut button) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity, FocusCause::Navigated);
                *bg = BackgroundColor(Color::srgb(0.35, 0.75, 0.35));
                *border = BorderColor::all(Color::srgb(1.0, 0.0, 0.0));
                button.set_changed(); // signal accessibility system
            }
            Interaction::Hovered => {
                input_focus.set(entity, FocusCause::Navigated);
                *bg = BackgroundColor(Color::srgb(0.25, 0.25, 0.25));
                *border = BorderColor::all(Color::WHITE);
                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
                *bg = BackgroundColor(Color::BLACK);
                *border = BorderColor::all(Color::BLACK);
                // No set_changed() for None — not required by the a11y system.
            }
        }
    }
}