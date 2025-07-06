use bevy::{ecs::system::RunSystemOnce, prelude::*};

use crate::{
    raft_out::{
        draw::RaftOutDrawPlugin,
        island::{RaftOutIslandPlugin, create_island},
        level::RaftOutLevelPlugin,
        player::RaftOutPlayerPlugin,
        raft::RaftOutRaftPlugin,
        trees::RaftOutTreesPlugin,
        waves::RaftOutWavesPlugin,
    },
    text_renderer::draw::BackgroundCharacter,
};

mod cell;
mod draw;
mod island;
mod level;
mod player;
mod raft;
mod trees;
mod waves;

pub struct RaftOutPlugin;

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
        ))
        .insert_resource(BackgroundCharacter {
            character: '~',
            color: ratatui::style::Color::Blue,
        })
        .add_systems(Startup, setup);
    }
}

fn setup(world: &mut World) {
    let _ = world.run_system_once(create_island);
}
