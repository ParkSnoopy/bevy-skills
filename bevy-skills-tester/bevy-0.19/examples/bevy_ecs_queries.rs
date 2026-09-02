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

// Sequential iteration. `&` for read, `&mut` for write.
fn move_things(time: Res<Time>, mut q: Query<(&Velocity, &mut Transform)>) {
    let dt = time.delta_secs();
    for (vel, mut tf) in &mut q {
        tf.translation += vel.0 * dt;
    }
}

// Change detection. `Changed<T>` triggers on insert OR mutation.
// `Added<T>` triggers only on insert.
fn on_health_changed(q: Query<(Entity, &Health), Changed<Health>>) {
    for (entity, hp) in &q {
        info!("entity {:?} now has {} hp", entity, hp.0);
    }
}

// Combined filters. `With`/`Without` constrain entities;
// `Or<(...)>` alternates over filters (not components).
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

// Parallel iteration. Use when N is large (>10k) and per-entity work is non-trivial.
// Cannot use `Commands` or external mutable state — task pool runs items in parallel.
fn integrate_in_parallel(mut q: Query<(&Velocity, &mut Transform)>) {
    q.par_iter_mut().for_each(|(vel, mut tf)| {
        tf.translation += vel.0 * 0.016;
    });
}

// Query lens: temporarily view a query as a narrower one. Useful for
// passing a stricter query into a helper without re-binding the system's
// SystemParam list.
#[allow(dead_code)]
fn use_lens(mut q: Query<(&mut Transform, &Velocity)>) {
    // Read-only narrowed view of just the Transform column.
    let mut lens = q.transmute_lens::<&Transform>();
    let _read_only: Query<&Transform> = lens.query();
}
