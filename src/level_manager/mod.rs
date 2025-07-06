use bevy::{ecs::system::SystemParam, platform::collections::HashMap, prelude::*};

pub struct LevelManagerPlugin;

impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelManagerInfo {
            current_world: 0,
            worlds: HashMap::new(),
        });
    }
}

#[derive(Resource)]
pub struct LevelManagerInfo {
    current_world: usize,
    worlds: HashMap<usize, Entity>,
}

#[derive(SystemParam)]
pub struct LevelManager<'w, 's> {
    #[doc(hidden)]
    commands: Commands<'w, 's>,
    #[doc(hidden)]
    level_manager_info: ResMut<'w, LevelManagerInfo>,
    #[doc(hidden)]
    children: Query<'w, 's, &'static Children>,
}

impl<'w, 's> LevelManager<'w, 's> {
    fn get_or_spawn(&mut self, world_index: usize) -> Entity {
        let maybe_existing = self.level_manager_info.worlds.get(&world_index);

        if let Some(existing) = maybe_existing {
            return *existing;
        }
        let new = self.commands.spawn_empty().id();
        self.level_manager_info.worlds.insert(world_index, new);
        new
    }

    fn get_or_spawn_current(&mut self) -> Entity {
        self.get_or_spawn(self.level_manager_info.current_world)
    }

    pub fn spawn_in_level(&mut self, world_index: usize, bundle: impl Bundle) -> EntityCommands {
        let world = self.get_or_spawn(world_index);

        self.commands.spawn((ChildOf(world), bundle))
    }

    pub fn spawn_in_current_level(&mut self, bundle: impl Bundle) -> EntityCommands {
        self.spawn_in_level(self.level_manager_info.current_world, bundle)
    }

    pub fn clear_current(&mut self) {
        let current_level = self.get_or_spawn_current();
        self.commands
            .entity(current_level)
            .despawn_related::<Children>();
    }
}
