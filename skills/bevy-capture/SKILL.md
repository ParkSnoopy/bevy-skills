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

`bevy_capture` 0.6.0 is the Bevy 0.19-compatible release. No local patch is needed. Its API returns a `RenderTarget` from `RenderTarget::target_headless`, and `Capture::start` accepts any `IntoEncoders` value.

The snippet below is verified against `bevy_capture = "0.6"` + `bevy = "0.19"`.

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
bevy_capture = "0.6"
```

```rust
//! `bevy-capture` skill — record a `Camera3d` to a PNG-per-frame sequence.
//!
//! `bevy_capture 0.6` tracks `bevy 0.19`. The old skill patch notes no
//! longer apply — the upstream crate now publishes a 0.19-compatible release.
//! API highlights (0.6):
//!   * `Capture::start(impl IntoEncoders)` — a single `Encoder` impl OR a
//!     tuple of them OR a `Vec<BoxedEncoder>`.
//!   * `RenderTarget::target_headless(w, h, &mut images)` returns a
//!     `RenderTarget` (not the old `(Camera, RenderTarget)` tuple) —
//!     `RenderTarget` is its own component.
//!   * `FramesEncoder::new(path)` writes PNGs to a directory; no system deps,
//!     no Cargo feature gate.
//!
//! Uses `FramesEncoder` because it has no external system dependency and no
//! OpenH264 binary license. `Mp4Openh264Encoder` / `Mp4FfmpegCliEncoder` /
//! `Mp4FfmpegCliPipeEncoder` are feature-gated in `bevy_capture`'s Cargo.toml.

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
    encoder::frames::FramesEncoder,
};

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
    // `bevy_capture`; `RenderTarget` is a separate component.
    let render_target = RenderTarget::target_headless(256, 256, &mut images);
    commands.spawn((Camera2d, render_target, CaptureBundle::default()));
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
```

Verified against `bevy = "0.19"` + `bevy_capture = "0.6"` — `cargo check --example bevy_capture` clean. The snippet lives at `skills-examples/examples/bevy_capture.rs`.

## Encoder choice

See [`references/encoders.md`](references/encoders.md) for the deep dive. Quick table:

| Encoder | Mechanism | System dep | Cargo feature | When to use |
|---|---|---|---|---|
| `Mp4Openh264Encoder` | In-process OpenH264 (downloaded at build) | None | `mp4_openh264` | CI without ffmpeg; single-binary distribution |
| `Mp4FfmpegCliEncoder` | Collects frames, shells out once at stop | `ffmpeg` on `$PATH` | `mp4_ffmpeg_cli` | Short clips, full ffmpeg codec control |
| `Mp4FfmpegCliPipeEncoder` | Long-running ffmpeg child, pipes per-frame | `ffmpeg` on `$PATH` | `mp4_ffmpeg_cli_pipe` | Long recordings, low memory |
| `FramesEncoder` | One PNG per frame into a directory | None | *(always available)* | Compositing, GIF pipelines, WASM |

## Gotchas

- **`Camera.target` is gone.** `RenderTarget` is a separate component. In `bevy_capture 0.6`, use `RenderTarget::target_headless(w, h, &mut images)` (returns a `RenderTarget`) or spawn `RenderTarget::Image(handle)` alongside `Camera3d`. See `bevy-cameras`.
- **Encoders are feature-gated.** `mp4_openh264`, `mp4_ffmpeg_cli`, and `mp4_ffmpeg_cli_pipe` are three *separate* features — enable each one you import. `FramesEncoder` is always available. Missing the feature gives a "cannot find type" compile error, not a friendly diagnostic.
- **OpenH264 license.** `Mp4Openh264Encoder` links Cisco's OpenH264 binary, distributed under Cisco's OBQI (royalty-covered for H.264 baseline). If your runtime-dependency policy forbids non-MIT/Apache binaries, pick one of the ffmpeg-CLI encoders instead.
- **ffmpeg-CLI encoders need `ffmpeg` on `$PATH`.** `Mp4FfmpegCliEncoder::new` and `Mp4FfmpegCliPipeEncoder::new` return `Result` — handle the spawn error, don't `.unwrap()`. `Mp4Openh264Encoder` and `FramesEncoder` have no system dependency.
- **Camera must be active and rendering.** `CaptureBundle` hooks into the render loop on that camera entity. Inactive or unsourced cameras emit no frames; capture appears to silently do nothing.
- **WASM:** only `FramesEncoder` works. The MP4 encoders shell out (ffmpeg) or rely on the OpenH264 native binary, neither of which exist in `wasm32-unknown-unknown`. See `bevy-wasm-webgpu` for the WASM build path.
- **Older `bevy_capture` forks.** The 0.6 release already targets Bevy 0.19 and uses the current message APIs; only carry `PollType`/`EventWriter` fixes when maintaining an older fork.

## See also

- `bevy-cameras` — camera spawning and the `RenderTarget`-as-component model that `bevy_capture` attaches to.
- `bevy-cargo-features` — picking Bevy feature flags alongside `bevy_capture`'s encoder gates.
- `bevy-wasm-webgpu` — WASM caveat: only `FramesEncoder` survives `wasm32` builds.
- [`references/encoders.md`](references/encoders.md) — encoder-by-encoder deep dive.
