//! `bevy-capture` skill — record a `Camera3d` to a PNG-per-frame sequence.
//!
//! `bevy_capture 0.6` tracks `bevy 0.19`. The 0.18 skill's patch notes no
//! longer apply — the upstream crate now publishes a 0.19-compatible release.
//! API highlights (0.6):
//!   * `Capture::start(impl IntoEncoders)` — a single `Encoder` impl OR a
//!     tuple of them OR a `Vec<BoxedEncoder>`.
//!   * `RenderTarget::target_headless(w, h, &mut images)` returns a
//!     `RenderTarget` (not the 0.18 `(Camera, RenderTarget)` tuple) —
//!     `RenderTarget` is its own component in 0.18+.
//!   * `FramesEncoder::new(path)` writes PNGs to a directory; no system deps,
//!     no Cargo feature gate.
//!
//! Uses `FramesEncoder` because it has no external system dependency and no
//! OpenH264 binary license. `Mp4Openh264Encoder` / `Mp4FfmpegCliEncoder` /
//! `Mp4FfmpegCliPipeEncoder` are feature-gated in `bevy_capture`'s Cargo.toml.

use bevy::camera::RenderTarget;
use bevy::prelude::*;
use bevy_capture::{
    encoder::frames::FramesEncoder, Capture, CaptureBundle, CapturePlugin,
    RenderTargetHeadless,
};
use std::fs;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CapturePlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, drive_capture)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // Spawn a 2D camera rendering into a headless off-screen image so we
    // have something to record without bringing windowing logic into the
    // example. `RenderTarget::target_headless` is an extension trait from
    // `bevy_capture`; `RenderTarget` is a separate component (0.18+).
    let render_target = RenderTarget::target_headless(256, 256, &mut images);
    commands.spawn((
        Camera2d,
        render_target,
        CaptureBundle::default(),
    ));
}

// Call `Capture::start` once we're ready, then `stop` after a few frames.
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
        // FramesEncoder: one PNG per frame into a directory; no encoding.
        capture.start(FramesEncoder::new("captures/frames"));
    }

    *frame += 1;

    // Stop flushes the encoder (calls `Encoder::finish` on drop). Guard with
    // `stopped` so `capture.stop()` is called only once — repeated calls may
    // double-flush a pipe or panic.
    if *frame >= 30 && !*stopped {
        *stopped = true;
        capture.stop();
    }
}