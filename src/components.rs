use bracket_lib::prelude::{ FontCharType };
use specs::prelude::*;
use specs_derive::*;
use bracket_lib::prelude::{RGB};

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CellState {
    Off = 0,
    On = 1, 
}