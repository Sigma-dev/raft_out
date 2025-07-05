use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_ratatui::RatatuiContext;
use ratatui::{Frame, layout::Rect, style::Stylize, widgets::Block};

#[derive(Resource, Default)]
pub struct Flags {
    pub debug: bool,
}

#[derive(Resource, Debug)]
pub struct TextRendererSize(pub UVec2);
pub struct TextRendererDrawPlugin;

impl Plugin for TextRendererDrawPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Flags>()
            .add_event::<DrawCharacter>()
            .add_systems(Update, draw);
    }
}

#[derive(Event)]
pub struct DrawCharacter {
    pub pos: IVec2,
    pub character: char,
    pub color: ratatui::style::Color,
}

#[derive(Resource)]
pub struct BackgroundCharacter {
    pub character: char,
    pub color: ratatui::style::Color,
}

fn draw(
    mut commands: Commands,
    maybe_background: Option<Res<BackgroundCharacter>>,
    mut ratatui: ResMut<RatatuiContext>,
    flags: Res<Flags>,
    diagnostics: Res<DiagnosticsStore>,
    mut draw_r: EventReader<DrawCharacter>,
) {
    let _ = ratatui.draw(|frame| {
        let area = debug_frame(frame, &flags, &diagnostics);
        let buffer = frame.buffer_mut();
        if let Some(background) = maybe_background {
            for y in area.top()..area.bottom() {
                for x in area.left()..area.right() {
                    if let Some(cell) = buffer.cell_mut((x, y)) {
                        cell.set_char(background.character);
                        cell.fg = background.color;
                    }
                }
            }
        }
        for draw in draw_r.read() {
            let middle = (
                (area.left() + area.right()) / 2,
                (area.top() + area.bottom()) / 2,
            );
            if let Some(cell) = buffer.cell_mut((
                (middle.0 as i32 + draw.pos.x) as u16,
                // Reverse y direction to match bevy's typical representation
                (middle.1 as i32 - draw.pos.y) as u16,
            )) {
                cell.set_char(draw.character);
                cell.fg = draw.color;
            }
        }
        commands.insert_resource(TextRendererSize(UVec2::new(
            area.width as u32,
            area.height as u32,
        )));
    });
}

pub fn debug_frame(frame: &mut Frame, flags: &Flags, diagnostics: &DiagnosticsStore) -> Rect {
    let mut block = Block::bordered().bg(ratatui::style::Color::Black);
    if flags.debug {
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed())
        {
            block = block.title_top(format!("[fps: {value:.0}]"));
        }
    }

    let inner = block.inner(frame.area());
    frame.render_widget(block, frame.area());

    inner
}
