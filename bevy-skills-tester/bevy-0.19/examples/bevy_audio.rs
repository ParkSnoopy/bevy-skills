use bevy::{
    audio::Volume,
    prelude::*,
};

#[derive(Component)]
struct Music;

fn start_music(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        Name::new("Exploration music"),
        Music,
        AudioPlayer::new(assets.load("audio/exploration.ogg")),
        PlaybackSettings::LOOP.with_volume(Volume::Decibels(-9.0)),
    ));
}

fn mute_music(mut sinks: Query<&mut AudioSink, With<Music>>) {
    for mut sink in &mut sinks {
        sink.mute();
    }
}

fn main() {}
