use bevy::prelude::*;

use crate::raft_out::{
    crabs::RaftOutCrabsPlugin, draw::RaftOutDrawPlugin, highscores::RaftOutHighScoresPlugin,
    island::RaftOutIslandPlugin, level::RaftOutLevelPlugin, level_intro::RaftOutLevelIntroPlugin,
    menu::RaftOutMenuPlugin, player::RaftOutPlayerPlugin, raft::RaftOutRaftPlugin,
    trees::RaftOutTreesPlugin, waves::RaftOutWavesPlugin,
};

mod cell;
mod crabs;
mod draw;
mod highscores;
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
    HighScores,
    LevelIntro,
    Level,
}

#[derive(Resource)]
pub struct PlayerLives(u32);

#[derive(Clone)]
pub struct HighScore {
    pub score: u32,
    pub level: u32,
}
#[derive(Resource)]
pub struct StoredData {
    pub high_scores: Vec<HighScore>,
}

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
            RaftOutHighScoresPlugin,
        ))
        .insert_resource(PlayerLives(1))
        .insert_resource(StoredData {
            high_scores: vec![HighScore {
                score: 1000,
                level: 1,
            }],
        })
        .insert_state(GameState::Menu);
    }
}
