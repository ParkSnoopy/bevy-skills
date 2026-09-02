use bevy::{
    asset::RenderAssetUsages,
    camera::RenderTarget,
    camera_controller::free_camera::{
        FreeCamera,
        FreeCameraPlugin,
    },
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FreeCameraPlugin) // adds the input wiring
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // 1. Main camera with the built-in free-look controller.
    commands.spawn((
        Camera3d::default(),
        FreeCamera::default(),
        Transform::from_xyz(0.0, 4.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 2. A texture target for an off-screen render pass (mini-map, portal).
    let size = bevy::render::render_resource::Extent3d {
        width: 512,
        height: 512,
        depth_or_array_layers: 1,
    };
    let mut image = Image::new_fill(
        size,
        bevy::render::render_resource::TextureDimension::D2,
        &[0; 4],
        bevy::render::render_resource::TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.texture_descriptor.usage = bevy::render::render_resource::TextureUsages::TEXTURE_BINDING
        | bevy::render::render_resource::TextureUsages::COPY_DST
        | bevy::render::render_resource::TextureUsages::RENDER_ATTACHMENT;
    let image_handle = images.add(image);

    // 3. A second camera that draws into the texture.
    //    RenderTarget is now a *separate* component, not Camera.target.
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: -1,
            ..default()
        }, // -1 = render before the main camera
        RenderTarget::Image(image_handle.into()),
        Transform::from_xyz(10.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
