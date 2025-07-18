use bevy::prelude::*;

use crate::{
    audio_manager::{AudioManager, PlayAudio2D},
    direction::Direction,
    raft_out::{
        GameState,
        cell::{Cell, SolidCell, WalkableCell},
    },
    text_renderer::input::TextRendererInputs,
};

pub struct RaftOutPlayerPlugin;

impl Plugin for RaftOutPlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<PlayerInteract>()
            .add_event::<PlayerInteractNoGround>()
            .add_systems(Update, move_player);
    }
}

#[derive(Component)]
pub struct Player {
    last_move: f32,
    pub facing: Direction,
}

#[derive(Component)]
pub struct CarryingWood;

#[derive(Event)]
pub struct PlayerInteract;

#[derive(Event)]
pub struct PlayerInteractNoGround {
    pub dir: Direction,
    pub pos: IVec2,
}

pub fn spawn_player(commands: &mut Commands, pos: IVec2) {
    commands.spawn((
        Cell::new(pos),
        Player {
            last_move: 0.,
            facing: Direction::Down,
        },
        StateScoped(GameState::Level),
    ));
}

fn move_player(
    time: Res<Time>,
    mut commands: Commands,
    mut audio_manager: AudioManager,
    pressed: Res<TextRendererInputs>,
    mut player_q: Query<(&mut Cell, &mut Player)>,
    cells: Query<(Entity, &Cell, Option<&WalkableCell>, Option<&SolidCell>), Without<Player>>,
    mut interaction_w: EventWriter<PlayerInteractNoGround>,
) {
    let Ok((mut player_cell, mut player)) = player_q.single_mut() else {
        return;
    };
    if time.elapsed_secs() < player.last_move + 0.2 {
        return;
    }
    let mut maybe_requested_move = None;
    if pressed.pressed(KeyCode::KeyW) {
        maybe_requested_move = Some(Direction::Up);
    }
    if pressed.pressed(KeyCode::KeyD) {
        maybe_requested_move = Some(Direction::Right);
    }
    if pressed.pressed(KeyCode::KeyS) {
        maybe_requested_move = Some(Direction::Down);
    }
    if pressed.pressed(KeyCode::KeyA) {
        maybe_requested_move = Some(Direction::Left);
    }
    if let Some(requested_move) = maybe_requested_move {
        let destination = player_cell.pos + requested_move.as_ivec2();
        player.facing = requested_move;

        if !cells
            .iter()
            .any(|(_, c, w, _)| c.pos == destination && w.is_some())
        {
            interaction_w.write(PlayerInteractNoGround {
                pos: destination,
                dir: requested_move.flipped(),
            });
            return;
        }
        player.last_move = time.elapsed_secs();
        if let Some((e, _, _, _)) = cells
            .iter()
            .find(|(_, c, _, s)| c.pos == destination && s.is_some())
        {
            commands.entity(e).trigger(PlayerInteract);
            return;
        }
        player_cell.pos = destination;
        audio_manager.play_sound(PlayAudio2D::new_once("sounds/sand.wav".to_owned()));
    }
}
