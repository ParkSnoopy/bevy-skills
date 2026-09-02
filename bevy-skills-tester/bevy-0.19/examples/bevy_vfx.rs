use bevy::prelude::*;
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
        .add_plugins((DefaultPlugins, HanabiPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let writer = ExprWriter::new();
    let position = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(1.0_f32).expr(),
        dimension: ShapeDimension::Volume,
    };
    let velocity = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(4.0_f32).expr(),
    };
    let lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        writer.lit(0.5_f32).uniform(writer.lit(1.5_f32)).expr(),
    );
    let age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.0_f32).expr());
    let gravity = AccelModifier::new(writer.lit(Vec3::new(0.0, -6.0, 0.0)).expr());

    let mut colors: bevy_hanabi::Gradient<Vec4> = bevy_hanabi::Gradient::new();
    colors.add_key(0.0, Vec4::new(4.0, 0.5, 0.0, 1.0));
    colors.add_key(0.5, Vec4::new(2.0, 1.0, 0.0, 0.8));
    colors.add_key(1.0, Vec4::new(0.5, 0.5, 0.5, 0.0));

    let effect = EffectAsset::new(
        16_384,
        SpawnerSettings::rate(200.0_f32.into()),
        writer.finish(),
    )
    .with_name("ember_burst")
    .init(position)
    .init(velocity)
    .init(lifetime)
    .init(age)
    .update(gravity)
    .render(ColorOverLifetimeModifier::new(colors));

    commands.spawn((
        Name::new("ember_burst"),
        ParticleEffect::new(effects.add(effect)),
        Transform::default(),
        Visibility::default(),
    ));
}
