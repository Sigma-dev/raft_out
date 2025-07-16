use bevy::prelude::*;

use crate::{
    direction::Direction,
    raft_out::{
        GameState,
        cell::Cell,
        island::IslandCell,
        level::{ExitPos, GameData},
        player::{CarryingWood, Player, PlayerInteractNoGround, spawn_player},
    },
    text_renderer::draw::{DrawCharacter, TextRendererSize},
};

pub struct RaftOutRaftPlugin;

impl bevy::prelude::Plugin for RaftOutRaftPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (draw_preview, handle_placing, draw_progress, move_raft),
        );
    }
}

#[derive(Component)]
pub struct Raft {
    pub facing: Direction,
}

#[derive(Component)]
pub struct RaftConstruction {
    required: u32,
    progress: u32,
}

#[derive(Component)]
pub struct MovingRaft {
    pub last_move: f32,
}

fn draw_preview(
    mut draw_w: EventWriter<DrawCharacter>,
    player_q: Query<(&Cell, &Player), With<CarryingWood>>,
    cells: Query<&Cell>,
) {
    let Ok((player_cell, player)) = player_q.single() else {
        return;
    };

    let target = player_cell.pos + player.facing.as_ivec2();
    if cells.iter().any(|c| c.pos == target) {
        return;
    }

    draw_w.write(DrawCharacter {
        pos: target,
        character: '#',
        color: ratatui::style::Color::Rgb(180, 100, 30),
    });
}

fn handle_placing(
    mut commands: Commands,
    game_data: Res<GameData>,
    mut rafts: Query<(Entity, &Cell, &mut RaftConstruction)>,
    cells: Query<&Cell>,
    mut interactions_r: EventReader<PlayerInteractNoGround>,
    player_q: Query<Entity, (With<CarryingWood>, With<Cell>, With<Player>)>,
) {
    for interaction in interactions_r.read() {
        let Ok(player) = player_q.single() else {
            return;
        };
        if let Some((e, _, mut raft)) = rafts.iter_mut().find(|(_, c, _)| c.pos == interaction.pos)
        {
            raft.progress += 1;
            if raft.progress >= raft.required {
                commands.entity(player).despawn();
                commands.entity(e).insert(MovingRaft { last_move: 0. });
                return;
            }
        } else {
            if cells.iter().any(|c| c.pos == interaction.pos) {
                return;
            }
            commands.spawn((
                Cell::new(interaction.pos),
                Raft {
                    facing: interaction.dir.flipped(),
                },
                RaftConstruction {
                    required: game_data.current_level + 2,
                    progress: 1,
                },
                StateScoped(GameState::Level),
            ));
        }
        commands.entity(player).remove::<CarryingWood>();
    }
}

fn draw_progress(
    mut draw_w: EventWriter<DrawCharacter>,
    rafts: Query<(&Cell, &Raft, &RaftConstruction), Without<MovingRaft>>,
) {
    for (raft_cell, raft, construction) in rafts {
        let progress = char::from_digit(construction.progress, 10).unwrap();
        let required = char::from_digit(construction.required, 10).unwrap();
        let reverse_order = [Direction::Up, Direction::Left].contains(&raft.facing);

        draw_w.write(DrawCharacter {
            pos: raft_cell.pos + raft.facing.as_ivec2(),
            character: if reverse_order { required } else { progress },
            color: ratatui::style::Color::White,
        });
        draw_w.write(DrawCharacter {
            pos: raft_cell.pos + raft.facing.as_ivec2() * 2,
            character: '/',
            color: ratatui::style::Color::White,
        });
        draw_w.write(DrawCharacter {
            pos: raft_cell.pos + raft.facing.as_ivec2() * 3,
            character: if reverse_order { progress } else { required },
            color: ratatui::style::Color::White,
        });
    }
}

fn move_raft(
    time: Res<Time>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_data: ResMut<GameData>,
    maybe_size: Option<Res<TextRendererSize>>,
    island_cells: Query<&Cell, With<IslandCell>>,
    mut raft_q: Query<(Entity, &mut Cell, &Raft, &mut MovingRaft), Without<IslandCell>>,
) {
    let Some(size) = maybe_size.map(|s| s.0.as_ivec2()) else {
        return;
    };
    let Ok((e, mut raft_cell, raft, mut moving_raft)) = raft_q.single_mut() else {
        return;
    };
    if time.elapsed_secs() < moving_raft.last_move + 0.05 {
        return;
    };
    let destination = raft_cell.pos + raft.facing.as_ivec2();
    if island_cells.iter().any(|c| c.pos == destination) {
        spawn_player(&mut commands, destination);
        commands.entity(e).despawn();
        return;
    }
    raft_cell.pos += raft.facing.as_ivec2();
    moving_raft.last_move = time.elapsed_secs();
    if raft_cell.pos.x.abs() >= size.x / 2 || raft_cell.pos.y.abs() >= size.y / 2 {
        commands.insert_resource(ExitPos(raft_cell.pos));
        game_data.current_level += 1;
        next_state.set(GameState::LevelIntro);
    }
}
