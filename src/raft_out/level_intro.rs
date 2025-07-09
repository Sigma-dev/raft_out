use bevy::prelude::*;

use crate::{
    raft_out::{GameState, PlayerLives},
    text_renderer::draw::{BackgroundCharacter, DrawCharacter},
};

pub struct RaftOutLevelIntroPlugin;

impl Plugin for RaftOutLevelIntroPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                draw_intro.run_if(in_state(GameState::LevelIntro)),
                end_intro,
            ),
        )
        .add_systems(OnEnter(GameState::LevelIntro), intro_enter);
    }
}

#[derive(Component)]
pub struct LevelIntro {
    start_time: f32,
}

fn intro_enter(time: Res<Time>, mut commands: Commands) {
    commands.insert_resource(BackgroundCharacter {
        character: ' ',
        color: ratatui::style::Color::White,
    });
    commands.spawn((
        LevelIntro {
            start_time: time.elapsed_secs(),
        },
        StateScoped(GameState::LevelIntro),
    ));
}

fn draw_intro(
    mut draw_w: EventWriter<DrawCharacter>,
    intro_q: Query<&LevelIntro>,
    lives: Res<PlayerLives>,
) {
    if intro_q.is_empty() {
        return;
    }
    for c in DrawCharacter::as_centered_text(IVec2::new(0, 0), format!("Lives: {}", lives.0)) {
        draw_w.write(c);
    }
}

fn end_intro(
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
    intro_q: Query<&LevelIntro>,
) {
    let Ok(intro) = intro_q.single() else {
        return;
    };
    if time.elapsed_secs() < intro.start_time + 2. {
        return;
    }
    next_state.set(GameState::Level);
}
