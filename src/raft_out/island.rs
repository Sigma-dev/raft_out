use bevy::prelude::*;

use crate::raft_out::cell::{Cell, WalkableCell};

pub struct RaftOutIslandPlugin;

impl bevy::prelude::Plugin for RaftOutIslandPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreStartup, create_island);
    }
}

#[derive(Component)]
pub struct IslandCell;

fn create_island(mut commands: Commands) {
    let sizes = [17, 23, 24, 27, 28, 28, 28, 27, 24, 17];

    for (y, size) in sizes.iter().enumerate() {
        for x in 0..*size {
            commands.spawn((
                Cell::new(IVec2::new(
                    (x - size / 2) as i32,
                    y as i32 - sizes.len() as i32 / 2,
                )),
                IslandCell,
                WalkableCell,
            ));
        }
    }
}
