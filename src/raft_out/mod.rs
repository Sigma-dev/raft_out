use bevy::prelude::*;

use crate::raft_out::{
    crabs::RaftOutCrabsPlugin, draw::RaftOutDrawPlugin, island::RaftOutIslandPlugin,
    level::RaftOutLevelPlugin, level_intro::RaftOutLevelIntroPlugin, menu::RaftOutMenuPlugin,
    player::RaftOutPlayerPlugin, raft::RaftOutRaftPlugin, trees::RaftOutTreesPlugin,
    waves::RaftOutWavesPlugin,
};

mod cell;
mod crabs;
mod draw;
mod island;
mod level;
mod level_intro;
mod menu;
mod player;
mod raft;
mod trees;
mod waves;

pub struct RaftOutPlugin;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, States)]
#[states(scoped_entities)]
enum GameState {
    Menu,
    LevelIntro,
    Level,
}

#[derive(Resource)]
pub struct PlayerLives(u32);

impl Plugin for RaftOutPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RaftOutDrawPlugin,
            RaftOutIslandPlugin,
            RaftOutPlayerPlugin,
            RaftOutTreesPlugin,
            RaftOutWavesPlugin,
            RaftOutRaftPlugin,
            RaftOutLevelPlugin,
            RaftOutCrabsPlugin,
            RaftOutLevelIntroPlugin,
            RaftOutMenuPlugin,
        ))
        .insert_resource(PlayerLives(1))
        .insert_state(GameState::Menu);
    }
}
