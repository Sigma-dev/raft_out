use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    direction::Direction,
    raft_out::{
        GameState, HighScore, PlayerLives, StoredData,
        cell::{Cell, SolidCell},
        island::IslandCell,
        level::{GameData, LevelData},
        player::Player,
    },
};

pub struct RaftOutCrabsPlugin;

impl bevy::prelude::Plugin for RaftOutCrabsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (spawn_crabs, move_crabs, handle_hurt).run_if(in_state(GameState::Level)),
        );
    }
}

#[derive(Component)]
pub struct Crab {
    last_move: f32,
}

fn spawn_crabs(
    time: Res<Time>,
    mut commands: Commands,
    game_data: Res<GameData>,
    player_q: Query<&Cell, (With<Player>, Without<Crab>)>,
    crabs: Query<&Cell, (With<Crab>, Without<IslandCell>)>,
    islands: Query<&Cell, With<IslandCell>>,
    level_start: Res<LevelData>,
) {
    let Ok(player) = player_q.single() else {
        return;
    };
    let crabs_amount = crabs.iter().len() as u32;
    let max_crabs = game_data.current_level;
    let spawn_interval = 2.;
    if crabs_amount >= max_crabs
        || time.elapsed_secs() < level_start.start_time + (crabs_amount + 1) as f32 * spawn_interval
    {
        return;
    }
    let mut vec = islands.iter().collect::<Vec<_>>();
    vec.shuffle(&mut thread_rng());
    let Some(pos) = vec
        .iter()
        .map(|c| c.pos)
        .filter(|p| p.distance_squared(player.pos) > 10)
        .find(|p| {
            Direction::iter()
                .map(|d| p + d.as_ivec2())
                .any(|pos| islands.iter().all(|c| c.pos != pos))
        })
    else {
        return;
    };
    commands.spawn((
        Cell::new(pos),
        Crab { last_move: 0. },
        StateScoped(GameState::Level),
    ));
}

fn move_crabs(
    time: Res<Time>,
    mut crabs: Query<(&mut Cell, &mut Crab)>,
    player_q: Query<&Cell, (With<Player>, Without<Crab>)>,
    solids: Query<&Cell, (With<SolidCell>, Without<Crab>)>,
    islands: Query<&Cell, (With<IslandCell>, Without<Crab>)>,
) {
    let Ok(player) = player_q.single() else {
        return;
    };
    for (mut crab_cell, mut crab) in crabs.iter_mut() {
        if time.elapsed_secs() < crab.last_move + 0.5 {
            return;
        }
        let mut possible_destinations = Direction::iter()
            .map(|d| d.as_ivec2())
            .map(|offset| crab_cell.pos + offset)
            .collect::<Vec<_>>();
        possible_destinations.sort_by_key(|a| -a.distance_squared(player.pos));
        for destination in possible_destinations {
            if solids.iter().any(|s| s.pos == destination) {
                continue;
            }
            if islands.iter().all(|s| s.pos != destination) {
                continue;
            }
            crab_cell.pos = destination;
            crab.last_move = time.elapsed_secs();
        }
    }
}

fn handle_hurt(
    mut next_state: ResMut<NextState<GameState>>,
    mut lives: ResMut<PlayerLives>,
    crabs: Query<&Cell, With<Crab>>,
    player_q: Query<&Cell, With<Player>>,
    game_data: Res<GameData>,
    mut high_score: ResMut<StoredData>,
) {
    let Ok(player) = player_q.single() else {
        return;
    };
    for crab in crabs {
        if crab.pos == player.pos {
            if lives.0 == 0 {
                high_score.high_scores.push(HighScore {
                    score: game_data.score,
                    level: game_data.current_level,
                });
                next_state.set(GameState::Menu);
            } else {
                lives.0 -= 1;
                next_state.set(GameState::LevelIntro);
            }
        }
    }
}
