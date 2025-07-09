use bevy::prelude::*;

use crate::raft_out::{
    GameState,
    cell::{Cell, WalkableCell},
};

pub struct RaftOutIslandPlugin;

impl bevy::prelude::Plugin for RaftOutIslandPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<IslandCreated>()
            .add_systems(OnEnter(GameState::Level), create_island);
    }
}

#[derive(Component)]
pub struct IslandCell;

#[derive(Event)]
pub struct IslandCreated;

pub fn create_island(mut commands: Commands, mut create_w: EventWriter<IslandCreated>) {
    let lines = [17, 23, 24, 27, 28, 28, 28, 27, 24, 17];

    for (y, size) in lines.iter().enumerate() {
        for x in 0..*size {
            commands.spawn((
                Cell::new(IVec2::new(
                    (x - size / 2) as i32,
                    y as i32 - lines.len() as i32 / 2,
                )),
                IslandCell,
                WalkableCell,
                StateScoped(GameState::Level),
            ));
        }
    }
    create_w.write(IslandCreated);
}
