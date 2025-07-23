#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn as_ivec2(&self) -> IVec2 {
        match self {
            Direction::Up => IVec2::Y,
            Direction::Right => IVec2::X,
            Direction::Down => IVec2::NEG_Y,
            Direction::Left => IVec2::NEG_X,
        }
    }

    pub fn as_vec2(&self) -> Vec2 {
        self.as_ivec2().as_vec2()
    }

    pub fn flipped(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }

    pub fn from_index(index: usize) -> Direction {
        match index {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Left,
            3 => Direction::Down,
            _ => panic!(),
        }
    }

    pub fn as_index(&self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }

    pub fn iter() -> impl Iterator<Item = Direction> {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
        .iter()
        .copied()
    }
}
