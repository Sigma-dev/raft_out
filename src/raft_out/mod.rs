use bevy::prelude::*;

use crate::{
    raft_out::{
        draw::RaftOutDrawPlugin, island::RaftOutIslandPlugin, player::RaftOutPlayerPlugin,
        trees::RaftOutTreesPlugin,
    },
    text_renderer::draw::BackgroundCharacter,
};

mod cell;
mod draw;
mod island;
mod player;
mod trees;

pub struct RaftOutPlugin;

impl Plugin for RaftOutPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RaftOutDrawPlugin,
            RaftOutIslandPlugin,
            RaftOutPlayerPlugin,
            RaftOutTreesPlugin,
        ))
        .insert_resource(BackgroundCharacter {
            character: '~',
            color: ratatui::style::Color::Blue,
        });
    }
}
