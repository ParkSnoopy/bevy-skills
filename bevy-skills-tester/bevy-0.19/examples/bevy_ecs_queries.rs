use bevy::prelude::*;

#[derive(Component, Default)]
struct Health(f32);

#[derive(Component, Default)]
struct Velocity(Vec3);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Update,
            (
                move_things,
                on_health_changed,
                damage_visible_enemies,
                integrate_in_parallel,
            ),
        )
        .run();
}

fn move_things(time: Res<Time>, mut q: Query<(&Velocity, &mut Transform)>) {
    let dt = time.delta_secs();
    for (vel, mut tf) in &mut q {
        tf.translation += vel.0 * dt;
    }
}

fn on_health_changed(q: Query<(Entity, &Health), Changed<Health>>) {
    for (entity, hp) in &q {
        info!("entity {:?} now has {} hp", entity, hp.0);
    }
}

fn damage_visible_enemies(
    mut q: Query<
        &mut Health,
        (
            With<Enemy>,
            Without<Player>,
            Or<(Added<Enemy>, Changed<Transform>)>,
        ),
    >,
) {
    for mut hp in &mut q {
        hp.0 -= 1.0;
    }
}

fn integrate_in_parallel(mut q: Query<(&Velocity, &mut Transform)>) {
    q.par_iter_mut().for_each(|(vel, mut tf)| {
        tf.translation += vel.0 * 0.016;
    });
}

#[allow(dead_code)]
fn use_lens(mut q: Query<(&mut Transform, &Velocity)>) {
    let mut lens = q.transmute_lens::<&Transform>();
    let _read_only: Query<&Transform> = lens.query();
}
