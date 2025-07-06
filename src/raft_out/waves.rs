use bevy::prelude::*;
use rand::{Rng, thread_rng};

use crate::{
    level_manager::LevelManager,
    raft_out::{cell::Cell, island::IslandCell},
    text_renderer::draw::TextRendererSize,
};

pub struct RaftOutWavesPlugin;

impl bevy::prelude::Plugin for RaftOutWavesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (spawn_waves, move_waves));
    }
}

#[derive(Component)]
pub struct Wave {
    last_move: f32,
}

fn spawn_waves(
    mut level_manager: LevelManager,
    maybe_size: Option<Res<TextRendererSize>>,
    waves_q: Query<&Cell, With<Wave>>,
) {
    if waves_q.iter().len() >= 25 {
        return;
    }

    let Some(half_size) = maybe_size.map(|s| s.0.as_ivec2() / 2) else {
        return;
    };

    let pos = IVec2::new(
        thread_rng().gen_range(-half_size.x..half_size.x),
        thread_rng().gen_range(-half_size.y..half_size.y),
    );
    level_manager.spawn_in_current_level((Cell::new(pos), Wave { last_move: 0. }));
}

fn move_waves(
    mut commands: Commands,
    time: Res<Time>,
    maybe_size: Option<Res<TextRendererSize>>,
    mut waves_q: Query<(Entity, &mut Cell, &mut Wave), Without<IslandCell>>,
    island_q: Query<&Cell, With<IslandCell>>,
) {
    let Some(half_size) = maybe_size.map(|s| s.0.as_ivec2() / 2) else {
        return;
    };
    for (e, mut wave_cell, mut wave) in waves_q.iter_mut() {
        if time.elapsed_secs() < wave.last_move + 0.2 {
            continue;
        }
        wave_cell.pos.x += 1;
        wave.last_move = time.elapsed_secs();
        if island_q.iter().any(|c| c.pos == wave_cell.pos) || wave_cell.pos.x > half_size.x {
            commands.entity(e).despawn();
        }
    }
}
