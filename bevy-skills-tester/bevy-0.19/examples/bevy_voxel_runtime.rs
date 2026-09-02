use std::collections::{
    BinaryHeap,
    HashMap,
    VecDeque,
};

use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct SectionCoord {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct WorkToken {
    coord: SectionCoord,
    revision: u64,
    epoch: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum WorkState {
    #[default]
    Idle,
    Queued,
    Running,
    RunningAndDirty,
    Ready,
}

#[derive(Resource, Default)]
struct RemeshRuntime {
    state: HashMap<SectionCoord, WorkState>,
    latest: HashMap<SectionCoord, WorkToken>,
    queued: BinaryHeap<PriorityEntry>,
    completed: VecDeque<MeshResult>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct PriorityEntry {
    // Include the token or queue generation so obsolete heap entries can be skipped.
    priority: i64,
    queue_generation: u64,
    token: WorkToken,
}

struct MeshResult {
    token: WorkToken,
    mesh: MeshPayload,
    collider: ColliderPayload,
}

struct MeshPayload;
struct ColliderPayload;

fn main() {}
