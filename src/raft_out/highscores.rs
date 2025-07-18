use bevy::prelude::*;

use crate::{
    audio_manager::{AudioManager, PlayAudio2D},
    raft_out::{GameState, StoredData},
    text_renderer::{
        draw::{BackgroundCharacter, DrawCharacter},
        input::TextRendererInputs,
    },
};

pub struct RaftOutHighScoresPlugin;

impl Plugin for RaftOutHighScoresPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, draw.run_if(in_state(GameState::HighScores)))
            .add_systems(OnEnter(GameState::HighScores), enter)
            .add_systems(
                Update,
                handle_inputs.run_if(in_state(GameState::HighScores)),
            );
    }
}

fn enter(mut commands: Commands) {
    commands.insert_resource(BackgroundCharacter {
        character: '~',
        color: ratatui::style::Color::Blue,
    });
}

fn draw(mut draw_w: EventWriter<DrawCharacter>, stored_data: Res<StoredData>) {
    let mut sorted_scores = stored_data.high_scores.clone();
    sorted_scores.sort_by_key(|s| s.score);
    sorted_scores.reverse();
    let max_scores = 5;
    for i in 0..max_scores {
        let score = sorted_scores.get(i).map(|s| s);
        let string = match score {
            Some(s) => format!("{} - {:3} (Level {})", i + 1, s.score, s.level + 1),
            None => format!("{} -", i + 1),
        };
        for c in DrawCharacter::as_centered_text(IVec2::new(0, i as i32 * -2), string) {
            draw_w.write(c);
        }
    }

    for c in DrawCharacter::as_centered_text(
        IVec2::new(0, max_scores as i32 * -2),
        "* Back to menu".to_string(),
    ) {
        draw_w.write(c);
    }
}

fn handle_inputs(
    mut audio_manager: AudioManager,
    mut next_state: ResMut<NextState<GameState>>,
    pressed: Res<TextRendererInputs>,
) {
    if pressed.just_pressed(KeyCode::Enter) {
        audio_manager.play_sound(PlayAudio2D::new_once("sounds/enter.wav".to_owned()));
        next_state.set(GameState::Menu);
    }
}
