use bevy::prelude::*;
use bevy_skills_tester::{
    build_i18n_plugin,
    setup_ui,
    switch_locale_on_keypress,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(build_i18n_plugin())
        .add_systems(Startup, setup_ui)
        .add_systems(Update, switch_locale_on_keypress)
        .run();
}
