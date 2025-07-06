use bevy::prelude::*;

use crate::{
    direction::Direction,
    level_manager::LevelManager,
    raft_out::{
        cell::Cell,
        island::create_island,
        raft::{MovingRaft, Raft},
    },
    text_renderer::draw::TextRendererSize,
};

pub struct RaftOutLevelPlugin;

impl bevy::prelude::Plugin for RaftOutLevelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<ExitLevel>().add_systems(
            PreUpdate,
            (
                handle_level_change.before(handle_level_change2),
                handle_level_change2,
            ),
        );
    }
}

#[derive(Event)]
pub struct ExitLevel {
    pub exit_pos: IVec2,
}

fn handle_level_change(mut level_manager: LevelManager, mut exits_r: EventReader<ExitLevel>) {
    for _ in exits_r.read() {
        level_manager.clear_current();
    }
}

fn handle_level_change2(
    mut commands: Commands,
    maybe_size: Option<Res<TextRendererSize>>,
    mut exits_r: EventReader<ExitLevel>,
) {
    let Some(size) = maybe_size else {
        return;
    };
    for exit in exits_r.read() {
        commands.run_system_cached(create_island);
        let exit_pos = exit.exit_pos;
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
        ));
    }
}
