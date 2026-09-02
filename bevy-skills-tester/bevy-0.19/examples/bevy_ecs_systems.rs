use bevy::{
    ecs::system::SystemParam,
    prelude::*,
};

#[derive(SystemSet, Hash, PartialEq, Eq, Clone, Debug)]
enum GameLoop {
    Input,
    Simulate,
    Render,
}

#[derive(States, Default, Hash, PartialEq, Eq, Clone, Debug)]
enum AppState {
    #[default]
    Loading,
    Playing,
}

#[derive(Resource, Default)]
struct Score(u32);

#[derive(Message)]
struct GoalScored {
    team: u8,
}

// Composite param: pass one argument, get four.
// 'w = world borrow; 's = system-local state borrow.
#[derive(SystemParam)]
struct GameCtx<'w, 's> {
    time: Res<'w, Time>,
    score: ResMut<'w, Score>,
    goals: MessageReader<'w, 's, GoalScored>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .add_message::<GoalScored>()
        .init_state::<AppState>()
        .configure_sets(
            Update,
            (GameLoop::Input, GameLoop::Simulate, GameLoop::Render).chain(),
        )
        // State schedule: fires once when entering Playing.
        .add_systems(OnEnter(AppState::Playing), spawn_level)
        // State schedule: fires once when leaving Playing.
        .add_systems(OnExit(AppState::Playing), despawn_level)
        .add_systems(Update, read_input.in_set(GameLoop::Input))
        .add_systems(
            Update,
            tally_goals
                .in_set(GameLoop::Simulate)
                .run_if(on_message::<GoalScored>),
        )
        .add_systems(Update, draw_hud.in_set(GameLoop::Render))
        .run();
}

#[derive(Component)]
struct LevelEntity;

fn spawn_level(mut commands: Commands) {
    commands.spawn((LevelEntity, Name::new("level")));
}

fn despawn_level(mut commands: Commands, query: Query<Entity, With<LevelEntity>>) {
    for e in &query {
        commands.entity(e).despawn();
    }
}

fn read_input(mut writer: MessageWriter<GoalScored>) {
    writer.write(GoalScored { team: 0 });
}

fn tally_goals(mut ctx: GameCtx) {
    let _ = ctx.time.delta_secs();
    for goal in ctx.goals.read() {
        ctx.score.0 += 1;
        info!("team {} scored, total {}", goal.team, ctx.score.0);
    }
}

fn draw_hud(score: Res<Score>) {
    let _ = score.0;
}
