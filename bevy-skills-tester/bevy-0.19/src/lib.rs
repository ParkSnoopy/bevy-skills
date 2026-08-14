use bevy::prelude::*;
use es_fluent::EsFluent;
use es_fluent_manager_bevy::{
    BevyFluentText,
    FluentText,
    I18nPlugin,
    LocaleChangeEvent,
    RequestedLanguageId,
};
use unic_langid::langid;

pub mod i18n;

#[derive(BevyFluentText, Clone, EsFluent)]
#[fluent(namespace = "ui")]
pub enum UiMessage {
    StartGame,
    Settings,
    QuitGame,
}

pub fn build_i18n_plugin() -> I18nPlugin {
    I18nPlugin::with_language(langid!("en"))
}

pub fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((FluentText::new(UiMessage::StartGame), Text::new("")));
}

pub fn switch_locale_on_keypress(
    keys: Res<ButtonInput<KeyCode>>,
    requested: Res<RequestedLanguageId>,
    mut locale_events: MessageWriter<LocaleChangeEvent>,
) {
    if keys.just_pressed(KeyCode::KeyL) {
        let next = if requested.0.to_string() == "en" {
            langid!("ko")
        } else {
            langid!("en")
        };
        locale_events.write(LocaleChangeEvent(next));
    }
}
