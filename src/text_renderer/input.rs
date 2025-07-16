use bevy::prelude::*;

#[cfg(not(feature = "windowed"))]
use bevy_ratatui::event::KeyEvent;

pub struct TextRendererInputPlugin;

#[derive(Resource, Debug)]
pub struct TextRendererInputs {
    pub pressed: Vec<KeyCode>,
    pub just_pressed: Vec<KeyCode>,
    pub previosuly_pressed: Vec<KeyCode>,
}

impl TextRendererInputs {
    pub fn clear(&mut self) {
        self.previosuly_pressed = self.pressed.clone();
        self.pressed.clear();
        self.just_pressed.clear();
    }

    pub fn just_pressed(&self, key: KeyCode) -> bool {
        self.just_pressed.contains(&key)
    }

    pub fn pressed(&self, key: KeyCode) -> bool {
        self.pressed.contains(&key)
    }

    pub fn add_pressed(&mut self, key: KeyCode) {
        if !self.previosuly_pressed.contains(&key) {
            self.just_pressed.push(key);
        }
        self.pressed.push(key);
    }
}

impl Plugin for TextRendererInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TextRendererInputs {
            pressed: vec![],
            just_pressed: vec![],
            previosuly_pressed: vec![],
        })
        .add_systems(PreUpdate, (clear_inputs, handle_input_system).chain());
    }
}

#[cfg(not(feature = "windowed"))]
pub fn handle_input_system(
    mut crossterm_input: EventReader<KeyEvent>,
    mut inputs: ResMut<TextRendererInputs>,
) {
    use bevy_ratatui::crossterm::event::KeyEventKind;
    for event in crossterm_input.read() {
        if event.kind == KeyEventKind::Press {
            if let bevy_ratatui::crossterm::event::KeyCode::Char('w') = event.code {
                inputs.add_pressed(KeyCode::KeyW);
            }
            if let bevy_ratatui::crossterm::event::KeyCode::Char('z') = event.code {
                inputs.add_pressed(KeyCode::KeyW);
            }
            if let bevy_ratatui::crossterm::event::KeyCode::Char('d') = event.code {
                inputs.add_pressed(KeyCode::KeyD);
            }
            if let bevy_ratatui::crossterm::event::KeyCode::Char('s') = event.code {
                inputs.add_pressed(KeyCode::KeyS);
            }
            if let bevy_ratatui::crossterm::event::KeyCode::Char('q') = event.code {
                inputs.add_pressed(KeyCode::KeyA);
            }
            if let bevy_ratatui::crossterm::event::KeyCode::Char('a') = event.code {
                inputs.add_pressed(KeyCode::KeyA);
            }
            if let bevy_ratatui::crossterm::event::KeyCode::Enter = event.code {
                inputs.add_pressed(KeyCode::Enter);
            }
        }
    }
}

#[cfg(feature = "windowed")]
pub fn handle_input_system(
    bevy_input: Res<ButtonInput<KeyCode>>,
    mut inputs: ResMut<TextRendererInputs>,
) {
    for &press in bevy_input.get_pressed() {
        inputs.add_pressed(press);
    }

    /*  for &press in bevy_input.get_just_pressed() {
        inputs.add_just_pressed(press);
    } */
}

pub fn clear_inputs(mut inputs: ResMut<TextRendererInputs>) {
    inputs.clear();
}
