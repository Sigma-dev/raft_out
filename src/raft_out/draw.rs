use bevy::prelude::*;

use crate::{
    raft_out::{cell::Cell, island::IslandCell, player::Player},
    text_renderer::draw::DrawCharacter,
};

pub struct RaftOutDrawPlugin;

impl bevy::prelude::Plugin for RaftOutDrawPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(bevy::prelude::Update, (draw_island, draw_player).chain());
    }
}

fn draw_island(
    mut draw_w: EventWriter<DrawCharacter>,
    island_cells: Query<&Cell, With<IslandCell>>,
) {
    for cell in island_cells.iter() {
        draw_w.write(DrawCharacter {
            pos: cell.pos,
            character: '#',
            color: ratatui::style::Color::Yellow,
        });
    }
}

fn draw_player(mut draw_w: EventWriter<DrawCharacter>, player: Query<&Cell, With<Player>>) {
    for cell in player.iter() {
        draw_w.write(DrawCharacter {
            pos: cell.pos,
            character: 'P',
            color: ratatui::style::Color::Green,
        });
    }
}
