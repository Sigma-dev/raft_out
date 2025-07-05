use bevy::prelude::*;

use crate::{
    raft_out::cell::{Cell, SolidCell, WalkableCell},
    text_renderer::input::TextRendererPressed,
};

pub struct RaftOutPlayerPlugin;

impl Plugin for RaftOutPlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<PlayerInteract>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

#[derive(Component)]
pub struct Player {
    last_move: f32,
}

#[derive(Event)]
pub struct PlayerInteract {
    dir: IVec2,
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((Cell::new(IVec2::ZERO), Player { last_move: 0. }));
}

fn move_player(
    time: Res<Time>,
    mut commands: Commands,
    mut pressed_r: EventReader<TextRendererPressed>,
    mut player_q: Query<(&mut Cell, &mut Player)>,
    cells: Query<(Entity, &Cell, Option<&WalkableCell>, Option<&SolidCell>), Without<Player>>,
) {
    let (mut player_cell, mut player) = player_q.single_mut().unwrap();
    if time.elapsed_secs() < player.last_move + 0.2 {
        return;
    }
    let mut maybe_requested_move = None;
    for pressed in pressed_r.read() {
        match pressed.key {
            KeyCode::KeyW => {
                maybe_requested_move = Some(IVec2::Y);
            }
            KeyCode::KeyD => {
                maybe_requested_move = Some(IVec2::X);
            }
            KeyCode::KeyS => {
                maybe_requested_move = Some(-IVec2::Y);
            }
            KeyCode::KeyA => {
                maybe_requested_move = Some(-IVec2::X);
            }
            _ => {}
        }
    }
    if let Some(requested_move) = maybe_requested_move {
        let destination = player_cell.pos + requested_move;
        if !cells
            .iter()
            .any(|(_, c, w, _)| c.pos == destination && w.is_some())
        {
            return;
        }
        player.last_move = time.elapsed_secs();
        if let Some((e, _, _, _)) = cells
            .iter()
            .find(|(_, c, _, s)| c.pos == destination && s.is_some())
        {
            commands.entity(e).trigger(PlayerInteract {
                dir: -requested_move,
            });
            return;
        }
        player_cell.pos = destination;
    }
}
