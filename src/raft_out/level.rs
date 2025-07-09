use bevy::prelude::*;

use crate::{
    direction::Direction,
    raft_out::{
        GameState,
        cell::Cell,
        player::spawn_player,
        raft::{MovingRaft, Raft},
    },
    text_renderer::draw::{BackgroundCharacter, TextRendererSize},
};

pub struct RaftOutLevelPlugin;

impl bevy::prelude::Plugin for RaftOutLevelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(CurrentLevel { index: 0 })
            .add_systems(OnEnter(GameState::Level), (handle_background, start_level));
    }
}

#[derive(Resource)]
pub struct CurrentLevel {
    pub index: u32,
}

#[derive(Component)]
pub struct LevelStart {
    pub start_time: f32,
}

#[derive(Resource)]
pub struct ExitPos(pub IVec2);

pub fn start_level(
    time: Res<Time>,
    mut commands: Commands,
    maybe_size: Option<Res<TextRendererSize>>,
    maybe_exit: Option<Res<ExitPos>>,
) {
    commands.spawn((
        LevelStart {
            start_time: time.elapsed_secs(),
        },
        StateScoped(GameState::Level),
    ));
    if let Some(exit_pos) = maybe_exit.map(|e| e.0) {
        let size = maybe_size.unwrap();
        let half_size = size.0.as_ivec2() / 2;
        let raft = if exit_pos.x >= half_size.x {
            (IVec2::new(-exit_pos.x, exit_pos.y), Direction::Right)
        } else if exit_pos.x <= -half_size.x {
            (IVec2::new(-exit_pos.x, exit_pos.y), Direction::Left)
        } else if exit_pos.y >= half_size.y {
            (IVec2::new(exit_pos.x, -exit_pos.y), Direction::Up)
        } else {
            (IVec2::new(exit_pos.x, -exit_pos.y), Direction::Down)
        };
        commands.spawn((
            Cell::new(raft.0),
            Raft { facing: raft.1 },
            MovingRaft { last_move: 0. },
            StateScoped(GameState::Level),
        ));
    } else {
        spawn_player(&mut commands, IVec2::ZERO);
    }
}

fn handle_background(mut commands: Commands) {
    commands.insert_resource(BackgroundCharacter {
        character: '~',
        color: ratatui::style::Color::Blue,
    });
}
