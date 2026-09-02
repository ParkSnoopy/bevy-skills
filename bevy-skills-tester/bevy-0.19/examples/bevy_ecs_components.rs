use bevy::prelude::*;

// 1. Plain components.
#[derive(Component, Default)]
struct Health(f32);

#[derive(Component)]
struct Velocity(Vec3);

// 2. Required components — spawning `Player` auto-spawns the rest.
//    `#[require]` accepts `Type` (Default), `Type(args)` (tuple constructor),
//    or `Type = expression`.
#[derive(Component)]
#[require(Health = Health(100.0), Velocity = Velocity(Vec3::ZERO), Transform)]
struct Player;

// 3. Sparse storage for components added/removed every frame (e.g. tags
//    flipped by gameplay). Default Table storage is faster to iterate.
#[derive(Component)]
#[component(storage = "SparseSet")]
struct Stunned;

// 4. An entity-targeted event reacted to by observers.
#[derive(EntityEvent)]
struct Damage {
    entity: Entity, // EntityEvent requires an `entity` field.
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

// Observer parameter is `On<E>`, not `Trigger<E>` (renamed in 0.17, PR #19596).
fn on_damage(damage: On<Damage>, mut query: Query<&mut Health>) {
    let event = damage.event();
    if let Ok(mut hp) = query.get_mut(event.entity) {
        hp.0 -= event.amount;
    }
}
