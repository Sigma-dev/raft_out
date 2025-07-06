use bevy::prelude::*;

use crate::{
    raft_out::{
        cell::Cell, island::IslandCell, level::CurrentLevel, player::Player, raft::Raft,
        trees::Tree, waves::Wave,
    },
    text_renderer::draw::{DrawCharacter, TextRendererSize},
};

pub struct RaftOutDrawPlugin;

impl bevy::prelude::Plugin for RaftOutDrawPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            bevy::prelude::Update,
            (
                draw_waves,
                draw_island,
                draw_trees,
                draw_player,
                draw_raft,
                draw_level,
            )
                .chain(),
        );
    }
}

fn draw_level(
    mut draw_w: EventWriter<DrawCharacter>,
    level: Res<CurrentLevel>,
    maybe_size: Option<Res<TextRendererSize>>,
) {
    let Some(top) = maybe_size.map(|s| (s.0.y / 2) as i32) else {
        return;
    };
    let string = format!("Level {}", level.0 + 1);
    for (i, c) in string.chars().enumerate() {
        draw_w.write(DrawCharacter {
            pos: IVec2::new(-((string.len() / 2) as i32) + i as i32, top + 1),
            character: c,
            color: ratatui::style::Color::White,
        });
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
            color: ratatui::style::Color::White,
        });
    }
}

fn draw_raft(mut draw_w: EventWriter<DrawCharacter>, raft: Query<&Cell, With<Raft>>) {
    for cell in raft.iter() {
        draw_w.write(DrawCharacter {
            pos: cell.pos,
            character: 'w',
            color: ratatui::style::Color::Rgb(90, 55, 40),
        });
    }
}
