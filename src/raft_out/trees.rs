use bevy::prelude::*;
use rand::{seq::IteratorRandom, thread_rng};

use crate::raft_out::{
    GameState,
    cell::{Cell, SolidCell},
    island::{IslandCell, IslandCreated},
    level::CurrentLevel,
    player::{CarryingWood, Player, PlayerInteract},
};

pub struct RaftOutTreesPlugin;

impl bevy::prelude::Plugin for RaftOutTreesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, spawn_trees);
    }
}

#[derive(Component)]
pub struct Tree;

fn spawn_trees(
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
    mut island_created_r: EventReader<IslandCreated>,
    island_q: Query<&Cell, With<IslandCell>>,
) {
    let tree_amount = current_level.index + 3;
    for _ in island_created_r.read() {
        let positions = island_q
            .iter()
            .map(|c| c.pos)
            .filter(|p| *p != IVec2::ZERO)
            .choose_multiple(&mut thread_rng(), tree_amount as usize);

        for pos in positions {
            commands.spawn((Cell::new(pos), SolidCell, Tree, StateScoped(GameState::Level))).observe(
                |trigger: Trigger<PlayerInteract>,
                 mut commands: Commands,
                 player_q: Query<Entity, (With<Player>, Without<CarryingWood>)>| {
                    let Ok(player) = player_q.single() else {
                        return;
                    };
                    commands.entity(trigger.target()).despawn();
                    commands.entity(player).insert(CarryingWood);
                },
            );
        }
    }
}
