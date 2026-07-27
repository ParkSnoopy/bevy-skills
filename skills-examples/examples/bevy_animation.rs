//! `bevy-animation` skill — procedural `AnimationClip`, `AnimationGraph`,
//! `AnimationTransitions::play`, `#[derive(AnimationEvent)]` + `On<E>` observer.
//!
//! Builds the clip in code (no glTF file needed), composes a graph, spawns
//! the player, and starts playback on any entity that gained an
//! `AnimationPlayer`. Also spawns a `Camera3d` so the app is visible when run.
//! `On<E>` (not `Trigger<E>`) — the rename landed in 0.17 and still holds in 0.19.

use bevy::animation::graph::AnimationNodeIndex;
use bevy::animation::{
    animated_field,
    animation_curves::{AnimatableCurve, AnimatableKeyframeCurve},
    AnimationEvent, AnimationTargetId,
};
use bevy::prelude::*;
use core::time::Duration;

#[derive(AnimationEvent, Clone)]
struct FootstepEvent {
    foot: u8,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, start)
        .add_observer(on_footstep)
        .run();
}

fn setup(
    mut commands: Commands,
    mut clips: ResMut<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // 1. Build a tiny procedural clip: a translation curve on "Hips" + an event.
    let bone = AnimationTargetId::from_name(&Name::new("Hips"));
    let tween = AnimatableKeyframeCurve::new([
        (0.0_f32, Vec3::ZERO),
        (0.5, Vec3::new(0.0, 1.0, 0.0)),
        (1.0, Vec3::ZERO),
    ])
    .expect("strictly-increasing times");
    let curve = AnimatableCurve::new(animated_field!(Transform::translation), tween);
    let mut clip = AnimationClip::default();
    clip.add_curve_to_target(bone, curve);
    clip.add_event(0.5, FootstepEvent { foot: 0 });
    let clip_handle = clips.add(clip);

    // 2. Compose a graph: root -> clip node.
    let mut graph = AnimationGraph::new();
    let root = graph.root;
    let _clip_node = graph.add_clip(clip_handle, 1.0, root);
    let graph_handle = graphs.add(graph);

    // 3. Spawn the player entity. (Bones come from a loaded glTF scene in
    //    real use; here the procedural clip is enough to exercise the API.)
    commands.spawn((
        Name::new("AnimationRoot"),
        AnimationPlayer::default(),
        AnimationGraphHandle(graph_handle),
        AnimationTransitions::new(),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn start(
    mut q: Query<(&mut AnimationTransitions, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    use bevy::animation::RepeatAnimation;
    for (mut tx, mut player) in &mut q {
        tx.play(
            &mut player,
            AnimationNodeIndex::new(1),
            Duration::from_millis(250),
        )
        .set_repeat(RepeatAnimation::Forever);
    }
}

// `On<E>` derefs to `&E`: access event fields directly. The firing entity
// is at `trigger.trigger().target` (AnimationEventTrigger::target).
fn on_footstep(trigger: On<FootstepEvent>) {
    let foot = trigger.foot;
    let _entity = trigger.trigger().target;
    let _ = foot;
}