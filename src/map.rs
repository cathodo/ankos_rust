use bracket_lib::prelude::{ RGB, BTerm, RandomNumberGenerator, to_cp437 };
use super::{ World, CellState };
use std::cmp::{max, min};

pub const MAPWIDTH : usize = 80;
pub const MAPHEIGHT: usize = 50;
pub const MAPCOUNT: usize = MAPWIDTH * MAPHEIGHT;

#[derive(Default)]
pub struct Map {
    pub cells : Vec<CellState>,
    pub width : i32,
    pub height: i32,
}

impl Map {
    pub fn xy_idx(x: i32, y: i32) -> usize {
        (y as usize * MAPWIDTH as usize) + x as usize
    }

    pub fn new() -> Map {
        let mut map = Map{
            cells: vec![CellState::Off; MAPCOUNT],
            width: MAPWIDTH as i32,
            height: MAPHEIGHT as i32,
        };

        //initial tile in center
        let seed_pos = Self::xy_idx(MAPWIDTH as i32/2, MAPHEIGHT as i32/2);
        map.cells[seed_pos] = CellState::On;

        // return
        map
    }
}

pub fn draw_map(ecs: &World, ctx: &mut BTerm) {
    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;
    for tile in map.cells.iter() {
        // Render a tile depending upon the tile type
        let glyph;
        let mut fg;
        let bg = RGB::from_f32(0., 0., 0.);
        match tile {
            CellState::Off => {
                fg = RGB::from_f32(0.5, 0.5, 0.5);
                glyph = to_cp437('.');
            }
            CellState::On => {
                fg = RGB::from_f32(0.0, 1.0, 0.0);
                glyph =  to_cp437('#');
            }
        }
        ctx.set(x, y, fg, bg, glyph);
        
        // Move the coordinates
        x += 1;
        if x > MAPWIDTH as i32 - 1 {
            x = 0;
            y += 1;
        }
    }
    
}