use bevy::prelude::*;

use crate::{
    raft_out::{
        draw::RaftOutDrawPlugin, island::RaftOutIslandPlugin, player::RaftOutPlayerPlugin,
        trees::RaftOutTreesPlugin, waves::RaftOutWavesPlugin,
    },
    text_renderer::draw::BackgroundCharacter,
};

mod cell;
mod draw;
mod island;
mod player;
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
        ))
        .insert_resource(BackgroundCharacter {
            character: '~',
            color: ratatui::style::Color::Blue,
        });
    }
}
