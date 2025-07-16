use bevy::prelude::*;

use crate::{
    raft_out::GameState,
    text_renderer::{
        draw::{BackgroundCharacter, DrawCharacter},
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
        character: ' ',
        color: ratatui::style::Color::White,
    });
    commands.spawn((
        Menu {
            options: vec![MenuOption::Start, MenuOption::HighScores, MenuOption::Exit],
            index: 0,
        },
        StateScoped(GameState::Menu),
    ));
}

fn draw(mut draw_w: EventWriter<DrawCharacter>, menu_q: Query<&Menu>) {
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
}

fn handle_inputs(
    mut exit: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<GameState>>,
    pressed: Res<TextRendererInputs>,
    mut menu_q: Query<&mut Menu>,
) {
    let Ok(mut menu) = menu_q.single_mut() else {
        return;
    };
    if pressed.just_pressed(KeyCode::KeyW) {
        menu.index = (menu.index + menu.options.len() - 1) % menu.options.len()
    }
    if pressed.just_pressed(KeyCode::KeyS) {
        menu.index = (menu.index + 1) % menu.options.len()
    }
    if pressed.just_pressed(KeyCode::Enter) {
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
