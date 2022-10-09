use bracket_lib::prelude::{ BTerm, RGB, to_cp437, BLACK, GRAY, GREEN, FontCharType };
use specs::*;
use specs_derive::*;
use super::{ SCREENWIDTH, SCREENHEIGHT, ScrollMode };

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * SCREENWIDTH) + x as usize
}

pub fn idx_xy(idx: usize) -> (i32, i32) {
    let x = idx % SCREENWIDTH;
    let y = idx / SCREENWIDTH;
    return (x as i32, y as i32)
}

#[derive(Component, Default, PartialEq, Clone, Copy)]
pub enum CellState {
    #[default]
    Off = 0,
    On = 1,
}

#[derive(Component, Clone, Copy)]
pub struct Cell {
    pub state: CellState,
    pub x: i32,
    pub y: i32,
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

impl Cell {
    pub fn new(state: CellState, x: i32, y: i32) -> Cell {
        match state {
            CellState::On => {
                Cell {
                    state: CellState::On,
                    x,
                    y,
                    glyph: to_cp437('#'),
                    fg: RGB::named(GREEN),
                    bg: RGB::named(BLACK),
                }
            },
            CellState::Off => {
                Cell {
                    state: CellState::Off,
                    x,
                    y,
                    glyph: to_cp437('.'),
                    fg: RGB::named(GRAY),
                    bg: RGB::named(BLACK),
                }
            }
        }
    }
}

#[derive(Component, Clone)]
pub struct CellGrid2d {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
    pub wrap: bool,
}

impl CellGrid2d {
    pub fn new(w: usize, h: usize, seeds: Vec<(i32, i32)>, wrap: bool) -> CellGrid2d {

        let mut v: Vec<Cell> = Vec::new();

        for idx in 0..w*h {
            let (x, y) = idx_xy(idx);
            if seeds.contains(&(x, y)){
                // add living cell
                v.push(Cell::new(CellState::On, x, y));
            } else {
                // add dead cell
                v.push(Cell::new(CellState::Off, x, y));
            }
        }

        CellGrid2d { 
            width: w,
            height: h,
            cells: v,
            wrap,
         }
    }

    pub fn step(&self) -> Self {
        let moore: Vec<(i32, i32)> = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            // skip center
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let w: i32 = self.width as i32;
        let h: i32 = self.height as i32;

        let buffer: Vec<Cell> = self.cells.clone();
        let mut new_cells: Vec<Cell> = Vec::new();
        // iter the cells to find new states
        for idx in 0..self.cells.len() {
            let (cell_x, cell_y) = idx_xy(idx); 
            let mut n = 0;
            // iter over all moore n(eighbours)
            for (m_x, m_y) in moore.iter() {
                let mut x = cell_x + m_x;
                let mut y = cell_y + m_y;
                // if wrap, adjust the neighbor position to other side
                // ctx has 0:width-1 and 0:height-1 coords, so we use >= on the upper bound 
                if self.wrap {
                    if x < 0 { x += w }
                    if x >= w { x -= w }
                    if y < 0 { y += h }
                    if y >= h { y -= h }
                    // check neighbour state (use buffer)
                    if buffer[xy_idx(x, y)].state == CellState::On { n += 1 }
                } else { 
                    // if not wrap, ignore illegal positions
                    if !(x < 0 || x >= w || y < 0 || y >= h) {
                        if buffer[xy_idx(x, y)].state == CellState::On { n += 1 }
                    }
                }
            }
            // after moore iter, decide new cell state
            let new_state: CellState;
            // currently, conway rules
            match buffer[xy_idx(cell_x, cell_y)].state {
                CellState::On => {
                    if n < 2 || n > 3 { new_state = CellState::Off } 
                    else { new_state = CellState::On }
                },
                CellState::Off => {
                    if n == 3 { new_state = CellState::On }
                    else { new_state = CellState::Off }
                },
            }

            new_cells.push(Cell::new(new_state, cell_x, cell_y));
        } 

        CellGrid2d { 
            cells: new_cells,
            width: self.width, 
            height: self.height,
            wrap: self.wrap,
        }
    }

    pub fn draw_cells(&self, ctx: &mut BTerm) {
        for c in &self.cells {
            ctx.set(c.x, c.y, c.fg, c.bg, c.glyph)
        }
    }
}

#[derive(Component, Clone)]
pub struct CellGrid1d {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
    pub wrap: bool,
    pub scroll: bool
}

fn to_digits(mut v: u64) -> Vec<u8> {
    let mut digits: Vec<u8> = Vec::with_capacity(20);

    while v > 0 {
        let n = (v % 10) as u8;
        v /= 10;
        digits.push(n);
    }
    digits
}

impl CellGrid1d {
    pub fn new(w: usize, h: usize, seeds: Vec<(i32, i32)>, wrap: bool, scroll: ScrollMode) -> CellGrid1d {

        let mut v: Vec<Cell> = Vec::new();

        for idx in 0..w*h {
            let (x, y) = idx_xy(idx);
            if seeds.contains(&(x, y)){
                // add living cell
                v.push(Cell::new(CellState::On, x, y));
            } else {
                // add dead cell
                v.push(Cell::new(CellState::Off, x, y));
            }
        }

        CellGrid1d { 
            width: w,
            height: h,
            cells: v,
            wrap,
            scroll,
         }
    }

    pub fn step(&self) -> Self {
        let state_informing_neighbours: Vec<(i32, i32)> = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ];
        let w: i32 = self.width as i32;
        let h: i32 = self.height as i32;

        let buffer = self.cells.clone();
        let mut new_cells: Vec<Cell> = Vec::new();
        // iter the cells to find new states
        for idx in 0..self.cells.len() {
            let (cell_x, cell_y) = idx_xy(idx); 
            let mut n = 0;
            // iter over all n(eighbours), hold record
            let mut state_records: Vec<CellState> = Vec::new();
            for (m_x, m_y) in state_informing_neighbours.iter() {
                let mut x = cell_x + m_x;
                let mut y = cell_y + m_y;
                // if wrap, adjust the neighbor position to other side
                // ctx has 0:width-1 and 0:height-1 coords, so we use >= on the upper bound 
                if self.wrap {
                    if x < 0 { x += w }
                    if x >= w { x -= w }
                    if y < 0 { y += h }
                    if y >= h { y -= h }
                    // check neighbour state (use buffer)
                    if buffer[xy_idx(x, y)].state == CellState::On { state_records.push(CellState::On); }
                    else { state_records.push(CellState::Off); }
                } else { 
                    // if not wrap, ignore illegal positions
                    if !(x < 0 || x >= w || y < 0 || y >= h) {
                        if buffer[xy_idx(x, y)].state == CellState::On { state_records.push(CellState::On); }
                        else { state_records.push(CellState::Off); }
                    }
                }
            }
            // compile state_records
            to_digits(v)
            // after moore iter, decide new cell state
            let new_state: CellState;
            // wolfram rules
            match buffer[xy_idx(cell_x, cell_y)].state {
                CellState::On => {},
                CellState::Off => {
                    match state_records 
                        1 => {},

                    }
                },
            }

            new_cells.push(Cell::new(new_state, cell_x, cell_y));
        } 

        CellGrid1d { 
            cells: new_cells,
            width: self.width, 
            height: self.height,
            wrap: self.wrap,
        }
    }

    pub fn draw_cells(&self, ctx: &mut BTerm) {
        for c in &self.cells {
            ctx.set(c.x, c.y, c.fg, c.bg, c.glyph)
        }
    }
}