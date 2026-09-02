use std::fs::{
    self,
    File,
};

use bevy::{
    app::{
        RunMode,
        ScheduleRunnerPlugin,
    },
    camera::RenderTarget,
    prelude::*,
    render::RenderPlugin,
    winit::WinitPlugin,
};
use bevy_capture::{
    Capture,
    CaptureBundle,
    CapturePlugin,
    RenderTargetHeadless,
    encoder::mp4_openh264::Mp4Openh264Encoder,
};

const WIDTH: u16 = 1280;
const HEIGHT: u16 = 720;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins
                .build()
                .disable::<WinitPlugin>()
                .set(RenderPlugin {
                    synchronous_pipeline_compilation: true,
                    ..default()
                }),
            ScheduleRunnerPlugin {
                run_mode: RunMode::Loop { wait: None },
            },
            CapturePlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, record)
        .run()
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn((
        Camera2d,
        RenderTarget::target_headless(WIDTH.into(), HEIGHT.into(), &mut images),
        CaptureBundle::default(),
    ));
}

fn record(
    mut captures: Query<&mut Capture>,
    mut waited_for_pipeline: Local<bool>,
    mut frame: Local<u32>,
) {
    // Allow the render pipeline one frame to become ready.
    if !*waited_for_pipeline {
        *waited_for_pipeline = true;
        return;
    }

    let Ok(mut capture) = captures.single_mut() else {
        return;
    };
    if !capture.is_capturing() && *frame == 0 {
        fs::create_dir_all("captures").expect("create captures directory");
        let encoder = Mp4Openh264Encoder::new(
            File::create("captures/run.mp4").expect("create output"),
            WIDTH,
            HEIGHT,
        )
        .expect("initialize OpenH264");
        capture.start(encoder);
    }

    *frame += 1;
    if *frame == 300 {
        capture.stop(); // drops the encoder and finalizes the MP4
    }
}
