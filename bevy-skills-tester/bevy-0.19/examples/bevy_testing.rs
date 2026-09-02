use bevy::{
    prelude::*,
    time::{
        TimePlugin,
        TimeUpdateStrategy,
    },
};

#[derive(Resource, Default)]
struct TickCount(u32);

fn count_tick(mut count: ResMut<TickCount>) {
    count.0 += 1;
}

#[test]
fn fixed_system_runs_exactly_three_times() {
    let mut app = App::new();
    app.add_plugins(TimePlugin)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(TimeUpdateStrategy::FixedTimesteps(1))
        .init_resource::<TickCount>()
        .add_systems(FixedUpdate, count_tick);

    // The first update initializes Bevy's real-time clock; it has zero delta.
    app.update();

    for _ in 0..3 {
        app.update();
    }

    assert_eq!(app.world().resource::<TickCount>().0, 3);
}

fn main() {}
