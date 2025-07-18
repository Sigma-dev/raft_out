use bevy::prelude::*;

use crate::{
    audio_manager::{AudioManager, PlayAudio2D},
    raft_out::GameState,
    text_renderer::{
        draw::{BackgroundCharacter, DrawCharacter, TextRendererSize},
        input::TextRendererInputs,
    },
};

pub struct RaftOutMenuPlugin;

impl Plugin for RaftOutMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (handle_inputs, draw.run_if(in_state(GameState::Menu))),
        )
        .add_systems(OnEnter(GameState::Menu), enter);
    }
}

#[derive(PartialEq, Eq)]
enum MenuOption {
    Start,
    HighScores,
    Exit,
}

impl MenuOption {
    fn text(&self) -> String {
        match self {
            MenuOption::Start => "Start Game",
            MenuOption::HighScores => "Highscores",
            MenuOption::Exit => "Exit",
        }
        .to_string()
    }
}

#[derive(Component)]
pub struct Menu {
    options: Vec<MenuOption>,
    index: usize,
}

fn enter(mut commands: Commands) {
    commands.insert_resource(BackgroundCharacter {
        character: '~',
        color: ratatui::style::Color::Blue,
    });
    commands.spawn((
        Menu {
            options: vec![MenuOption::Start, MenuOption::HighScores, MenuOption::Exit],
            index: 0,
        },
        StateScoped(GameState::Menu),
    ));
}

fn draw(
    mut draw_w: EventWriter<DrawCharacter>,
    menu_q: Query<&Menu>,
    maybe_size: Option<Res<TextRendererSize>>,
) {
    let Some(size) = maybe_size else {
        return;
    };
    let Ok(menu) = menu_q.single() else {
        return;
    };
    for (i, option) in menu.options.iter().enumerate() {
        let selected = menu.index == menu.options.iter().position(|o| o == option).unwrap();
        for c in DrawCharacter::as_centered_text(
            IVec2::new(0, i as i32 * -2),
            format!("{} {}", if selected { "*" } else { "" }, option.text()),
        ) {
            draw_w.write(c);
        }
    }
    let title = vec![
        vec![
            '#', '#', ' ', ' ', ' ', ' ', '#', ' ', ' ', ' ', '#', '#', '#', ' ', ' ', '#', '#',
            '#', ' ', ' ', ' ', '#', '#', '#', ' ', ' ', '#', ' ', '#', ' ', ' ', '#', '#', '#',
        ],
        vec![
            '#', ' ', '#', ' ', ' ', '#', ' ', '#', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#',
            ' ', ' ', ' ', ' ', '#', ' ', '#', ' ', ' ', '#', ' ', '#', ' ', ' ', ' ', '#', ' ',
        ],
        vec![
            '#', '#', ' ', ' ', ' ', '#', '#', '#', ' ', ' ', '#', '#', ' ', ' ', ' ', ' ', '#',
            ' ', ' ', ' ', ' ', '#', ' ', '#', ' ', ' ', '#', ' ', '#', ' ', ' ', ' ', '#', ' ',
        ],
        vec![
            '#', ' ', '#', ' ', ' ', '#', ' ', '#', ' ', ' ', '#', ' ', ' ', ' ', ' ', ' ', '#',
            ' ', ' ', ' ', ' ', '#', '#', '#', ' ', ' ', '#', '#', '#', ' ', ' ', ' ', '#', ' ',
        ],
    ];
    let scale = (size.0.y / 15) as i32;
    for (i, line) in title.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            for y in 0..scale {
                for x in 0..scale {
                    let config = if *c == ' ' {
                        ('~', ratatui::style::Color::Blue)
                    } else {
                        (*c, ratatui::style::Color::Rgb(90, 55, 40))
                    };

                    draw_w.write(DrawCharacter {
                        pos: IVec2::new(
                            -(line.len() as i32 * scale / 2) + j as i32 * scale + x as i32,
                            5 * scale - (i as i32 * scale + y as i32),
                        ),
                        character: config.0,
                        color: config.1,
                    });
                }
            }
        }
    }
}

fn handle_inputs(
    mut audio_manager: AudioManager,
    mut exit: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<GameState>>,
    pressed: Res<TextRendererInputs>,
    mut menu_q: Query<&mut Menu>,
) {
    let Ok(mut menu) = menu_q.single_mut() else {
        return;
    };
    let maybe_move: Option<i32> = if pressed.just_pressed(KeyCode::KeyW) {
        Some(-1)
    } else if pressed.just_pressed(KeyCode::KeyS) {
        Some(1)
    } else {
        None
    };

    if let Some(move_input) = maybe_move {
        audio_manager.play_sound(PlayAudio2D::new_once("sounds/select.wav".to_owned()));
        menu.index = (((menu.index + menu.options.len()) as i32 + move_input)
            % menu.options.len() as i32) as usize
    }

    if pressed.just_pressed(KeyCode::Enter) {
        audio_manager.play_sound(PlayAudio2D::new_once("sounds/enter.wav".to_owned()));
        match menu.options[menu.index] {
            MenuOption::Start => {
                next_state.set(GameState::LevelIntro);
            }
            MenuOption::HighScores => {
                next_state.set(GameState::HighScores);
            }
            MenuOption::Exit => {
                exit.write(AppExit::Success);
            }
        }
    }
}
