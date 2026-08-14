---
name: bevy-capture
description: Use when adding `CapturePlugin` to record a `Camera3d` or `Camera2d` to video, spawning `CaptureBundle` on a camera entity, calling `Capture::start` with `Mp4Openh264Encoder` for in-process MP4 encoding, using `Mp4FfmpegCliEncoder` or `Mp4FfmpegCliPipeEncoder` for ffmpeg-backed output, or writing per-frame PNGs with `FramesEncoder` in Bevy 0.19.
license: MIT
compatibility: opencode,claude-code,cursor
metadata:
  tier: "4"
  area: render
  bevy_version: "0.19"
---

# bevy-capture — recording Bevy 0.19 cameras to MP4 or PNG sequences

## Compatibility status

`bevy_capture` 0.6.0 natively depends on `bevy = "^0.19.0"`; no source patch is required. The 0.6 API exposes `RenderTargetHeadless` for constructing the off-screen `RenderTarget` component used below.

## When to use this skill

- Recording a gameplay clip or cinematic from a `Camera3d` or `Camera2d`.
- Generating MP4 files for trailers or automated screenshot tests.
- Capturing a PNG-per-frame sequence for offline compositing or GIF export.
- Choosing between in-process H.264 (`Mp4Openh264Encoder`) and ffmpeg-CLI encoders (`Mp4FfmpegCliEncoder`, `Mp4FfmpegCliPipeEncoder`).
- Hitting "ffmpeg not found" or "cannot find type `Mp4Openh264Encoder`" because a Cargo feature wasn't enabled.
- Wanting to stop and flush a recording mid-session via `Capture::stop()`.

## Canonical pattern

```toml
# Cargo.toml
[dependencies]
bevy = "0.19"
# Pick the features for the encoders you use; FramesEncoder needs none.
bevy_capture = { version = "0.6.0", features = ["mp4_openh264"] }
```

```rust
use bevy::{camera::RenderTarget, prelude::*};
use bevy_capture::{
    Capture, CaptureBundle, CapturePlugin, RenderTargetHeadless,
    encoder::mp4_openh264::Mp4Openh264Encoder,
};
use std::fs;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CapturePlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, drive_capture)
        .run();
}

// ① Spawn a camera with `CaptureBundle`. For an off-screen recording,
//    construct the separate `RenderTarget` component with
//    `RenderTarget::target_headless`.
fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    commands.spawn((
        Camera2d, // or Camera3d
        RenderTarget::target_headless(1920, 1080, &mut images),
        CaptureBundle::default(),
    ));
}

// ② Call `Capture::start(encoder)` once you're ready to record.
//    The render loop encodes every subsequent frame until `stop()`.
fn drive_capture(
    mut q: Query<&mut Capture>,
    mut started: Local<bool>,
    mut stopped: Local<bool>,
    mut frame: Local<u32>,
) {
    let Ok(mut capture) = q.single_mut() else { return };

    if !*started {
        *started = true;
        fs::create_dir_all("captures").ok();

        // Mp4Openh264Encoder — in-process H.264, no shell-out or system ffmpeg.
        capture.start(
            Mp4Openh264Encoder::new(
                fs::File::create("captures/out.mp4").unwrap(), 1920, 1080,
            ).expect("openh264 init"),
        );
    }

    *frame += 1;

    // ③ Stop flushes the encoder (calls `Encoder::finish` on drop).
    //    Guard with `stopped` so `capture.stop()` is called only once —
    //    repeated calls may double-flush a pipe or panic.
    if *frame >= 300 && !*stopped {
        *stopped = true;
        capture.stop();
    }
}
```

The pattern follows the published `bevy_capture = "0.6.0"` example for `bevy = "0.19"`.

## Encoder choice

See [`references/encoders.md`](references/encoders.md) for the deep dive. Quick table:

| Encoder | Mechanism | System dep | Cargo feature | When to use |
|---|---|---|---|---|
| `Mp4Openh264Encoder` | In-process OpenH264 (downloaded at build) | None | `mp4_openh264` | CI without ffmpeg; single-binary distribution |
| `Mp4FfmpegCliEncoder` | Collects frames, shells out once at stop | `ffmpeg` on `$PATH` | `mp4_ffmpeg_cli` | Short clips, full ffmpeg codec control |
| `Mp4FfmpegCliPipeEncoder` | Long-running ffmpeg child, pipes per-frame | `ffmpeg` on `$PATH` | `mp4_ffmpeg_cli_pipe` | Long recordings, low memory |
| `FramesEncoder` | One PNG per frame into a directory | None | *(always available)* | Compositing, GIF pipelines, WASM |

## Gotchas

- **`RenderTarget` is a separate component.** In `bevy_capture` 0.6, call `RenderTarget::target_headless(w, h, &mut images)` and spawn the returned component alongside `Camera2d` or `Camera3d`. See `bevy-cameras`.
- **Encoders are feature-gated.** `mp4_openh264`, `mp4_ffmpeg_cli`, and `mp4_ffmpeg_cli_pipe` are three *separate* features — enable each one you import. `FramesEncoder` is always available. Missing the feature gives a "cannot find type" compile error, not a friendly diagnostic.
- **OpenH264 license.** `Mp4Openh264Encoder` links Cisco's OpenH264 binary, distributed under Cisco's OBQI (royalty-covered for H.264 baseline). If your runtime-dependency policy forbids non-MIT/Apache binaries, pick one of the ffmpeg-CLI encoders instead.
- **ffmpeg-CLI encoders need `ffmpeg` on `$PATH`.** `Mp4FfmpegCliEncoder::new` and `Mp4FfmpegCliPipeEncoder::new` return `Result` — handle the spawn error, don't `.unwrap()`. `Mp4Openh264Encoder` and `FramesEncoder` have no system dependency.
- **Camera must be active and rendering.** `CaptureBundle` hooks into the render loop on that camera entity. Inactive or unsourced cameras emit no frames; capture appears to silently do nothing.
- **WASM:** only `FramesEncoder` works. The MP4 encoders shell out (ffmpeg) or rely on the OpenH264 native binary, neither of which exist in `wasm32-unknown-unknown`. See `bevy-wasm-webgpu` for the WASM build path.

## See also

- `bevy-cameras` — camera spawning and the `RenderTarget`-as-component model that `bevy_capture` attaches to.
- `bevy-cargo-features` — picking Bevy feature flags alongside `bevy_capture`'s encoder gates.
- `bevy-wasm-webgpu` — WASM caveat: only `FramesEncoder` survives `wasm32` builds.
- [`references/encoders.md`](references/encoders.md) — encoder-by-encoder deep dive.
