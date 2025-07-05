use bevy::prelude::*;

#[derive(Component)]
pub struct Cell {
    pub pos: IVec2,
}

#[derive(Component)]
pub struct WalkableCell;

#[derive(Component)]
pub struct SolidCell;

impl Cell {
    pub fn new(pos: IVec2) -> Cell {
        Cell { pos }
    }
}
