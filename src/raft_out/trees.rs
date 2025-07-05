use bevy::prelude::*;
use rand::{seq::IteratorRandom, thread_rng};

use crate::raft_out::{
    cell::{Cell, SolidCell},
    island::IslandCell,
    player::PlayerInteract,
};

pub struct RaftOutTreesPlugin;

impl bevy::prelude::Plugin for RaftOutTreesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_trees);
    }
}

#[derive(Component)]
pub struct Tree;

fn spawn_trees(mut commmands: Commands, island_q: Query<&Cell, With<IslandCell>>) {
    let positions = island_q
        .iter()
        .map(|c| c.pos)
        .filter(|p| *p != IVec2::ZERO)
        .choose_multiple(&mut thread_rng(), 3);

    for pos in positions {
        commmands.spawn((Cell::new(pos), SolidCell, Tree)).observe(
            |trigger: Trigger<PlayerInteract>, mut commands: Commands| {
                commands.entity(trigger.target()).despawn()
            },
        );
    }
}
