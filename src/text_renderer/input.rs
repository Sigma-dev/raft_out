use bevy::prelude::*;

#[cfg(not(feature = "windowed"))]
use bevy_ratatui::event::KeyEvent;

pub struct TextRendererInputPlugin;

#[derive(Event)]
pub struct TextRendererPressed {
    pub key: KeyCode,
}

#[derive(Event)]
pub struct TextRendererJustPressed {
    pub key: KeyCode,
}

impl Plugin for TextRendererInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TextRendererPressed>()
            .add_event::<TextRendererJustPressed>()
            .add_systems(Update, handle_input_system);
    }
}

#[cfg(not(feature = "windowed"))]
pub fn handle_input_system(
    mut crossterm_input: EventReader<KeyEvent>,
    mut pressed_w: EventWriter<TextRendererPressed>,
    mut just_pressed_w: EventWriter<TextRendererJustPressed>,
) {
    use bevy_ratatui::crossterm::event::KeyEventKind;
    for event in crossterm_input.read() {
        if event.kind == KeyEventKind::Press {
            if let bevy_ratatui::crossterm::event::KeyCode::Char('w') = event.code {
                pressed_w.write(TextRendererPressed { key: KeyCode::KeyW });
            }
            if let bevy_ratatui::crossterm::event::KeyCode::Char('z') = event.code {
                pressed_w.write(TextRendererPressed { key: KeyCode::KeyW });
            }
            if let bevy_ratatui::crossterm::event::KeyCode::Char('d') = event.code {
                pressed_w.write(TextRendererPressed { key: KeyCode::KeyD });
            }
            if let bevy_ratatui::crossterm::event::KeyCode::Char('s') = event.code {
                pressed_w.write(TextRendererPressed { key: KeyCode::KeyS });
            }
            if let bevy_ratatui::crossterm::event::KeyCode::Char('q') = event.code {
                pressed_w.write(TextRendererPressed { key: KeyCode::KeyA });
            }
            if let bevy_ratatui::crossterm::event::KeyCode::Char('a') = event.code {
                pressed_w.write(TextRendererPressed { key: KeyCode::KeyA });
            }
        }
        if event.kind == KeyEventKind::Repeat {
            if let bevy_ratatui::crossterm::event::KeyCode::Char('w') = event.code {
                just_pressed_w.write(TextRendererJustPressed { key: KeyCode::KeyW });
            }
        }
    }
}

#[cfg(feature = "windowed")]
pub fn handle_input_system(
    bevy_input: Res<ButtonInput<KeyCode>>,
    mut pressed_w: EventWriter<TextRendererPressed>,
    mut just_pressed_w: EventWriter<TextRendererJustPressed>,
) {
    for &press in bevy_input.get_pressed() {
        pressed_w.write(TextRendererPressed { key: press });
    }

    for &press in bevy_input.get_just_pressed() {
        just_pressed_w.write(TextRendererJustPressed { key: press });
    }
}
