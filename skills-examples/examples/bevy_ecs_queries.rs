//! `bevy-ecs-queries` skill — `Query` filters, change detection, `par_iter_mut`, lens.

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
        .add_systems(Startup, populate)
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

fn populate(mut commands: Commands) {
    commands.spawn((Camera3d::default(), Transform::from_xyz(0.0, 4.0, 8.0)));
    for i in 0..5 {
        commands.spawn((
            Enemy,
            Health(100.0),
            Velocity(Vec3::new(i as f32, 0.0, 0.0)),
            Transform::default(),
        ));
    }
}

// Sequential iteration: `&` read, `&mut` write.
fn move_things(time: Res<Time>, mut q: Query<(&Velocity, &mut Transform)>) {
    let dt = time.delta_secs();
    for (vel, mut tf) in &mut q {
        tf.translation += vel.0 * dt;
    }
}

// Change detection: `Changed<T>` fires on insert or mutation.
fn on_health_changed(q: Query<(Entity, &Health), Changed<Health>>) {
    for (entity, hp) in &q {
        info!("entity {entity:?} now has {} hp", hp.0);
    }
}

// Combined filters: `With`/`Without` constrain entities; `Or<(...)>`
// alternates over filters (not raw component types).
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

// Parallel iteration. Can't use `Commands` or external mutable state
// — task pool runs items in parallel.
fn integrate_in_parallel(mut q: Query<(&Velocity, &mut Transform)>) {
    q.par_iter_mut().for_each(|(vel, mut tf)| {
        tf.translation += vel.0 * 0.016;
    });
}

// Query lens: narrow a query to a stricter shape without re-binding.
#[allow(dead_code)]
fn use_lens(mut q: Query<(&mut Transform, &Velocity)>) {
    let mut lens = q.transmute_lens::<&Transform>();
    let _read_only: Query<&Transform> = lens.query();
}
