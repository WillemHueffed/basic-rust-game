use sdl2::rect::{Point, Rect};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct KeyboardControlled;

/// The current position of a given entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Point);

/// The current speed and direction of a given entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub speed: i32,
    pub direction: Direction,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
    /// The specific spritesheet to render from
    pub spritesheet: usize,
    /// The current region of the spritesheet to be rendered
    pub region: Rect,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct MovementAnimation {
    // The current frame in the animation of the direction this entity is moving in
    pub current_frame: usize,
    pub up_frames: Vec<Sprite>,
    pub down_frames: Vec<Sprite>,
    pub left_frames: Vec<Sprite>,
    pub right_frames: Vec<Sprite>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Player {
    pub position: Point,
    pub sprite: Rect,
    pub speed: i32,
    pub direction: Direction,
    pub current_frame: i32,
}
