use bevy::{
    diagnostic::{
        Diagnostic,
        DiagnosticPath,
        Diagnostics,
        RegisterDiagnostic,
    },
    prelude::*,
};

const REMESH_QUEUE_DEPTH: DiagnosticPath = DiagnosticPath::const_new("voxel/remesh_queue_depth");

#[derive(Resource, Default)]
struct RemeshQueue {
    valid_jobs: usize,
}

struct VoxelDiagnosticsPlugin;

impl Plugin for VoxelDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RemeshQueue>()
            .register_diagnostic(Diagnostic::new(REMESH_QUEUE_DEPTH).with_suffix(" sections"))
            .add_systems(Update, record_voxel_diagnostics);
    }
}

fn record_voxel_diagnostics(queue: Res<RemeshQueue>, mut diagnostics: Diagnostics) {
    diagnostics.add_measurement(&REMESH_QUEUE_DEPTH, || queue.valid_jobs as f64);
}

fn main() {}
