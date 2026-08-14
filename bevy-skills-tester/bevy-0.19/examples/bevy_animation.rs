use core::time::Duration;

use bevy::{
    animation::{
        AnimationEvent,
        AnimationTargetId,
        animated_field,
        animation_curves::{
            AnimatableCurve,
            AnimatableKeyframeCurve,
        },
    },
    prelude::*,
};

#[derive(AnimationEvent, Clone)]
struct FootstepEvent {
    foot: u8,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut clips: ResMut<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let walk: Handle<AnimationClip> = asset_server.load("models/character.glb#Animation0");
    let bone = AnimationTargetId::from_name(&Name::new("Hips"));
    let tween = AnimatableKeyframeCurve::new([
        (0.0_f32, Vec3::ZERO),
        (0.5, Vec3::new(0.0, 1.0, 0.0)),
        (1.0, Vec3::ZERO),
    ])
    .expect("strictly-increasing times");
    let curve = AnimatableCurve::new(animated_field!(Transform::translation), tween);
    let mut proc = AnimationClip::default();
    proc.add_curve_to_target(bone, curve);
    proc.add_event(0.5, FootstepEvent { foot: 0 });
    let proc = clips.add(proc);

    const MASK_GROUP_0_BIT: u64 = 1 << 0;
    let mut graph = AnimationGraph::new();
    let root = graph.root;
    let _walk_node = graph.add_clip(walk, 1.0, root);
    let additive = graph.add_additive_blend(0.5, root);
    let _proc_node = graph.add_clip_with_mask(proc, MASK_GROUP_0_BIT, 1.0, additive);

    commands.spawn((
        Name::new("AnimationRoot"),
        AnimationPlayer::default(),
        AnimationGraphHandle(graphs.add(graph)),
        AnimationTransitions::new(),
    ));
}

fn start(mut q: Query<(&mut AnimationTransitions, &mut AnimationPlayer), Added<AnimationPlayer>>) {
    use bevy::animation::{
        RepeatAnimation,
        graph::AnimationNodeIndex,
    };
    for (mut tx, mut player) in &mut q {
        tx.play(
            &mut player,
            AnimationNodeIndex::new(1),
            Duration::from_millis(250),
        )
        .set_repeat(RepeatAnimation::Forever);
    }
}

fn on_footstep(trigger: On<FootstepEvent>) {
    let foot = trigger.foot;
    let _entity = trigger.trigger().target;
    let _ = foot;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, start)
        .add_observer(on_footstep)
        .run();
}
