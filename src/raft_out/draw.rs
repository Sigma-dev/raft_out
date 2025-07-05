use bevy::prelude::*;

use crate::{
    raft_out::{cell::Cell, island::IslandCell, player::Player, trees::Tree, waves::Wave},
    text_renderer::draw::DrawCharacter,
};

pub struct RaftOutDrawPlugin;

impl bevy::prelude::Plugin for RaftOutDrawPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            bevy::prelude::Update,
            (draw_waves, draw_island, draw_trees, draw_player).chain(),
        );
    }
}

fn draw_waves(mut draw_w: EventWriter<DrawCharacter>, wave_cells: Query<&Cell, With<Wave>>) {
    for cell in wave_cells.iter() {
        draw_w.write(DrawCharacter {
            pos: cell.pos,
            character: '~',
            color: ratatui::style::Color::Cyan,
        });
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

fn draw_trees(mut draw_w: EventWriter<DrawCharacter>, player: Query<&Cell, With<Tree>>) {
    for cell in player.iter() {
        draw_w.write(DrawCharacter {
            pos: cell.pos,
            character: 'T',
            color: ratatui::style::Color::Rgb(90, 55, 40),
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
