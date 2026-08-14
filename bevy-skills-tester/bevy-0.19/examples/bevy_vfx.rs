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

    let writer = ExprWriter::new();
    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(1.0_f32).expr(),
        dimension: ShapeDimension::Volume,
    };
    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: writer.lit(4.0_f32).expr(),
    };
    let init_lifetime = SetAttributeModifier::new(
        Attribute::LIFETIME,
        writer.lit(0.5_f32).uniform(writer.lit(1.5_f32)).expr(),
    );
    let init_age = SetAttributeModifier::new(Attribute::AGE, writer.lit(0.0_f32).expr());
    let update_gravity = AccelModifier::new(writer.lit(Vec3::new(0.0, -6.0, 0.0)).expr());

    let mut color: bevy_hanabi::Gradient<Vec4> = bevy_hanabi::Gradient::new();
    color.add_key(0.0, Vec4::new(4.0, 0.5, 0.0, 1.0));
    color.add_key(0.5, Vec4::new(2.0, 1.0, 0.0, 0.8));
    color.add_key(1.0, Vec4::new(0.5, 0.5, 0.5, 0.0));
    let render_color = ColorOverLifetimeModifier::new(color);
    let spawner = SpawnerSettings::rate(200.0_f32.into());
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

    commands.spawn((
        Name::new("ember_burst"),
        ParticleEffect::new(handle),
        Transform::default(),
        Visibility::default(),
    ));
}

mod shader_effects {
    use bevy::{
        prelude::*,
        render::render_resource::AsBindGroup,
        shader::ShaderRef,
    };

    #[derive(Asset, AsBindGroup, TypePath, Clone)]
    pub struct FlickerMaterial {
        #[uniform(0)]
        time: f32,
        #[uniform(0)]
        intensity: f32,
    }

    impl Material for FlickerMaterial {
        fn fragment_shader() -> ShaderRef {
            "shaders/flicker.wgsl".into()
        }
    }

    pub fn add_plugin(app: &mut App) {
        app.add_plugins(MaterialPlugin::<FlickerMaterial>::default());
    }

    pub fn update_flicker(
        time: Res<Time>,
        mut materials: ResMut<Assets<FlickerMaterial>>,
        q: Query<&MeshMaterial3d<FlickerMaterial>>,
    ) {
        for handle in &q {
            if let Some(mut mat) = materials.get_mut(handle) {
                mat.time = time.elapsed_secs();
            }
        }
    }

    pub fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<FlickerMaterial>>,
    ) {
        commands.spawn((
            Mesh3d(meshes.add(Rectangle::new(2.0, 2.0))),
            MeshMaterial3d(materials.add(FlickerMaterial {
                time: 0.0,
                intensity: 2.5,
            })),
            Transform::default(),
        ));
    }
}
