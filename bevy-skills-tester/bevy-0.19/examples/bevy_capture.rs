use std::fs;

use bevy::{
    camera::RenderTarget,
    prelude::*,
};
use bevy_capture::{
    Capture,
    CaptureBundle,
    CapturePlugin,
    RenderTargetHeadless,
    encoder::mp4_openh264::Mp4Openh264Encoder,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CapturePlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, drive_capture)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn((
        Camera2d,
        RenderTarget::target_headless(1920, 1080, &mut images),
        CaptureBundle::default(),
    ));
}

fn drive_capture(
    mut q: Query<&mut Capture>,
    mut started: Local<bool>,
    mut stopped: Local<bool>,
    mut frame: Local<u32>,
) {
    let Ok(mut capture) = q.single_mut() else {
        return;
    };

    if !*started {
        *started = true;
        fs::create_dir_all("captures").ok();
        capture.start(
            Mp4Openh264Encoder::new(fs::File::create("captures/out.mp4").unwrap(), 1920, 1080)
                .expect("openh264 init"),
        );
    }

    *frame += 1;
    if *frame >= 300 && !*stopped {
        *stopped = true;
        capture.stop();
    }
}
