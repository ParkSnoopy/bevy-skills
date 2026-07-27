//! `bevy-vfx` skill — `bevy_hanabi 0.19` GPU particle effect on the main world.
//!
//! GPU particles need WebGPU on WASM targets; this example targets native only.
//! `bevy_hanabi::Gradient` name-collides with `bevy::prelude::Gradient`
//! (`bevy_ui`'s CSS gradient enum) — import from `bevy_hanabi::prelude` selectively
//! and qualify `Gradient` instead of pulling it in via the prelude glob.

use bevy::prelude::*;
// Do NOT `use bevy_hanabi::prelude::*` — `Gradient` collides with
// `bevy::prelude::Gradient` (bevy_ui's CSS gradient enum).
use bevy_hanabi::prelude::{
    AccelModifier,
    Attribute,
    ColorOverLifetimeModifier,
    EffectAsset,
    ExprWriter,
    HanabiPlugin,
    ParticleEffect,
    SetAttributeModifier,
    SetPositionSphereModifier,
    SetVelocitySphereModifier,
    ShapeDimension,
    SpawnerSettings,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HanabiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // ExprWriter accumulates WGSL expression nodes into a Module.
    // lit() returns WriterExpr — call .expr() to get the ExprHandle modifiers need.
    let writer = ExprWriter::new();

    // Init: scatter particles across a sphere's volume + push outward.
    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(1.0_f32).expr(),
        dimension: ShapeDimension::Volume,
    };
    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(4.0_f32).expr(),
    };

    // LIFETIME required to recycle particles; AGE required because
    // ColorOverLifetimeModifier reads age/lifetime.
    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        writer.lit(0.5_f32).uniform(writer.lit(1.5_f32)).expr(),
    );
    let init_age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.0_f32).expr());

    // Update: constant downward acceleration.
    let update_gravity = AccelModifier::new(writer.lit(Vec3::new(0.0, -6.0, 0.0)).expr());

    // Render: red -> orange -> transparent over lifetime.
    // Fully qualified: bevy_hanabi::Gradient (NOT bevy::prelude::Gradient).
    let mut color: bevy_hanabi::Gradient<Vec4> = bevy_hanabi::Gradient::new();
    color.add_key(0.0, Vec4::new(4.0, 0.5, 0.0, 1.0));
    color.add_key(0.5, Vec4::new(2.0, 1.0, 0.0, 0.8));
    color.add_key(1.0, Vec4::new(0.5, 0.5, 0.5, 0.0));
    let render_color = ColorOverLifetimeModifier::new(color);

    // Spawner: 200 particles/sec continuous stream. NOTE: Spawner::rate
    // does NOT exist in 0.19 — use SpawnerSettings::rate(n.into()).
    let spawner = SpawnerSettings::rate(200.0_f32.into());

    // finish() consumes the writer, producing the Module passed to
    // EffectAsset::new.
    let module = writer.finish();

    let effect = EffectAsset::new(16384, spawner, module)
        .with_name("ember_burst")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .init(init_age)
        .update(update_gravity)
        .render(render_color);
    let handle = effects.add(effect);

    // ParticleEffect is a bare component in 0.19 — no ParticleEffectBundle.
    // `#[require(CompiledParticleEffect, ...)]` fills in the rest automatically.
    commands.spawn((
        Name::new("ember_burst"),
        ParticleEffect::new(handle),
        Transform::default(),
        Visibility::default(),
    ));
}
