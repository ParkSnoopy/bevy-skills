use bevy::prelude::*;

#[derive(Component, Default)]
struct Health(f32);

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
#[require(Health = Health(100.0), Velocity = Velocity(Vec3::ZERO), Transform)]
struct Player;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct Stunned;

#[derive(EntityEvent)]
struct Damage {
    entity: Entity,
    amount: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, deal_damage)
        .add_observer(on_damage)
        .run();
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(Player);
}

fn deal_damage(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in &query {
        commands.trigger(Damage {
            entity,
            amount: 10.0,
        });
    }
}

fn on_damage(damage: On<Damage>, mut query: Query<&mut Health>) {
    let event = damage.event();
    if let Ok(mut hp) = query.get_mut(event.entity) {
        hp.0 -= event.amount;
    }
}
