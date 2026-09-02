use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const PHYSICS_HZ: f64 = 60.0;
const PHYSICS_DT: f32 = 1.0 / PHYSICS_HZ as f32;

#[derive(Component)]
struct Player;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GamePhysics {
    Drive,
    React,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(PHYSICS_HZ))
        .insert_resource(TimestepMode::Fixed {
            dt: PHYSICS_DT,
            substeps: 1,
        })
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default().in_fixed_schedule(),
            RapierDebugRenderPlugin::default(),
        ))
        .configure_sets(
            FixedUpdate,
            (
                GamePhysics::Drive.before(PhysicsSet::SyncBackend),
                GamePhysics::React.after(PhysicsSet::Writeback),
            ),
        )
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, drive_player.in_set(GamePhysics::Drive))
        .add_systems(FixedUpdate, react_to_collisions.in_set(GamePhysics::React));
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(8.0, 6.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // A collider without RigidBody is fixed. Cuboid arguments are half-extents.
    commands.spawn((
        Name::new("Ground"),
        Collider::cuboid(8.0, 0.25, 8.0),
        Transform::from_xyz(0.0, -0.25, 0.0),
    ));

    commands.spawn((
        Name::new("Player"),
        Player,
        RigidBody::Dynamic,
        Collider::ball(0.5),
        Velocity::linear(Vec3::new(1.5, 0.0, 0.0)),
        ActiveEvents::COLLISION_EVENTS,
        Transform::from_xyz(0.0, 4.0, 0.0),
    ));
}

fn drive_player(mut bodies: Query<&mut Velocity, With<Player>>) {
    for mut velocity in &mut bodies {
        velocity.linear.x = 1.5;
    }
}

fn react_to_collisions(mut events: MessageReader<CollisionEvent>) {
    for event in events.read() {
        if let CollisionEvent::Started(a, b, _) = event {
            let _pair = (*a, *b);
        }
    }
}
