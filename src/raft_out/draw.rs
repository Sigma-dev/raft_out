use bevy::prelude::*;

use crate::{
    raft_out::{
        GameState, cell::Cell, crabs::Crab, island::IslandCell, level::GameData, player::Player,
        raft::Raft, trees::Tree, waves::Wave,
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
                draw_crabs,
                draw_player,
                draw_raft,
                draw_level.run_if(in_state(GameState::Level)),
            )
                .chain(),
        );
    }
}

fn draw_level(
    mut draw_w: EventWriter<DrawCharacter>,
    game_data: Res<GameData>,
    maybe_size: Option<Res<TextRendererSize>>,
) {
    let Some(size) = maybe_size.map(|s| s.0.as_ivec2() / 2) else {
        return;
    };
    for i in -size.x..size.x {
        draw_w.write(DrawCharacter {
            pos: IVec2::new(i, size.y),
            character: ' ',
            color: ratatui::style::Color::White,
        });
    }
    for c in DrawCharacter::as_centered_text(
        IVec2::new(-10, size.y),
        format!("Level {}", game_data.current_level + 1),
    ) {
        draw_w.write(c);
    }

    for c in DrawCharacter::as_centered_text(
        IVec2::new(10, size.y),
        format!("Score {}", game_data.score),
    ) {
        draw_w.write(c);
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
            character: ':',
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
            character: '#',
            color: ratatui::style::Color::Rgb(90, 55, 40),
        });
    }
}

fn draw_crabs(mut draw_w: EventWriter<DrawCharacter>, crabs: Query<&Cell, With<Crab>>) {
    for cell in crabs.iter() {
        draw_w.write(DrawCharacter {
            pos: cell.pos,
            character: 'H',
            color: ratatui::style::Color::Red,
        });
    }
}
