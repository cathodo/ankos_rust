use bracket_lib::prelude::{ BTerm, RGB, to_cp437, BLACK, GRAY, GREEN, FontCharType, console };
use specs::*;
use specs_derive::*;
use super::{ SCREENWIDTH, SCREENHEIGHT, Mode, ScrollMode };

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
pub struct CellGrid {
    pub mode: Mode,
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
    pub wrap: bool,
    pub scroll: ScrollMode,
    pub w_line: usize,
}

impl CellGrid {
    pub fn new(mode: Mode, w: usize, h: usize, seeds: Vec<(i32, i32)>, wrap: bool, scroll: ScrollMode) -> CellGrid {
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

        CellGrid { 
            mode,
            width: w,
            height: h,
            cells: v,
            wrap,
            scroll,
            w_line: 1,
         }
    }

    pub fn step(&self) -> Self {
        match self.mode {
            Mode::Conway => { return self.conway_step() },
            Mode::Wolfram => { return self.wolfram_step() },
        };
    }

    // might need to calibrate this if I change number of states at any point
    fn to_digits(v: Vec<CellState>) -> String {
        let mut digits = String::new();

        for i in v {
            match i {
                CellState::On => { digits.push_str("1") }
                CellState::Off => { digits.push_str("0") }
            }
        }
        digits
    }

    fn match_rule(statebin: &str) -> CellState {
        let new_state: CellState;

        // currently rule 110
        match statebin {
            "111" => { new_state = CellState::Off },
            "110" => { new_state = CellState::On },
            "101" => { new_state = CellState::On },
            "100" => { new_state = CellState::Off },
            "011" => { new_state = CellState::On },
            "010" => { new_state = CellState::On },
            "001" => { new_state = CellState::On },
            "000" => { new_state = CellState::Off },
            _ => { new_state = CellState::Off }
        }

        new_state
    }

    fn null_step(&self) -> Self {

        let mut new_cells: Vec<Cell> = Vec::new();
        for idx in 0..self.cells.len() {
            let (cell_x, cell_y) = idx_xy(idx);
            new_cells.push(Cell::new(self.cells[idx].state, cell_x, cell_y))
        }


        CellGrid { 
            mode: self.mode,
            cells: new_cells,
            width: self.width, 
            height: self.height,
            wrap: self.wrap,
            scroll: self.scroll,
            w_line: self.w_line,
        }
    }

    fn shift_all_cells(&self, buffer: &Vec<Cell>, shift_x: i32, shift_y: i32) -> Vec<Cell> {
        let w: i32 = self.width as i32;
        let h: i32 = self.height as i32;
        let mut shifted_cells: Vec<Cell> = Vec::new();
        
        for idx in 0..buffer.len() {
            let (old_x, old_y) = idx_xy(idx);
            let (mut new_x, mut new_y) = (old_x + shift_x, old_y + shift_y);
            // wrap coords for cells which fall off the map
            if new_x < 0 { new_x += w }
            if new_x >= w { new_x -= w }
            if new_y < 0 { new_y += h }
            if new_y >= h { new_y -= h }
            // move the state
            shifted_cells.push(Cell::new(buffer[idx].state, new_x, new_y));  
        }

        // change the vector order (idx)
        shifted_cells.sort_by(|a, b| xy_idx(a.x, a.y).cmp(&xy_idx(b.x, b.y)));

        shifted_cells
    }

    fn wolfram_step(&self) -> Self {
        let state_informing_neighbours: Vec<(i32, i32)> = vec![
            (-1, -1),
            (0, -1),
            (1, -1),
        ];
        let w: i32 = self.width as i32;
        let h: i32 = self.height as i32;

        let mut buffer: Vec<Cell> = self.cells.clone();
        let mut new_cells: Vec<Cell> = self.cells.clone();

        // iter only cells which are in w_line
        for idx in (self.w_line*self.width)..((self.w_line+1)*self.width) {
            let mut new_state = CellState::Off; //buffer[idx].state;
            let (cell_x, cell_y) = idx_xy(idx); 
            // iter over all n(eighbours), hold record
            let mut state_records: Vec<CellState> = Vec::new();
            for (m_x, m_y) in state_informing_neighbours.iter() {
                let mut x = cell_x + m_x;
                let mut y = cell_y + m_y;
                // if wrap, adjust the neighbor position to other side
                // ctx has 0:width-1 and 0:height-1 coords, so we use >= on the upper bound 
                // x axis code still associated with wrap function, y axis code now associated with ScrollMode param
                match self.scroll {
                    ScrollMode::Loop => {
                        if y < 0 { y += h }
                        if y >= h { y -= h }
                    },
                    _ => {}
                }
                if self.wrap {
                    if x < 0 { x += w }
                    if x >= w { x -= w }
                        // check neighbour state (use buffer)
                        if buffer[xy_idx(x, y)].state == CellState::On { state_records.push(CellState::On); }
                        else { state_records.push(CellState::Off); }
                } else { 
                    // if not wrap, ignore illegal positions
                    if !(x < 0 || x >= w) {
                        // check neighbour state (use buffer)
                        if buffer[xy_idx(x, y)].state == CellState::On { state_records.push(CellState::On); }
                        else { state_records.push(CellState::Off); }
                    }
                }
            }
            // compile state_records
            let state_binary = Self::to_digits(state_records);

            // after moore iter, decide new cell state
            match buffer[xy_idx(cell_x, cell_y)].state {
                CellState::On => {},
                CellState::Off => {
                    new_state = Self::match_rule(state_binary.as_str())
                }
            }

            // new cells are made the same way as the buffer so we don't need to compute them all each time
            // only the w_line
            new_cells[idx] = Cell::new(new_state, cell_x, cell_y);
        } 

        // iterate the w line so next tick computes the line below
        let mut new_w_line = self.w_line+1;
        // adjust line depending on scoll params
        match self.scroll {
            ScrollMode::Stop => { if new_w_line >= self.height { new_w_line -= 1; } }
            ScrollMode::Loop => { if new_w_line >= self.height { new_w_line = 0; } }
            ScrollMode::Shift => { 
                if new_w_line >= self.height { 
                    new_w_line -= 1; 
                    new_cells = self.shift_all_cells(&new_cells, 0, -1)
                } 
            }
        }
       
        // return
        CellGrid { 
            mode: self.mode,
            cells: new_cells,
            width: self.width, 
            height: self.height,
            wrap: self.wrap,
            scroll: self.scroll,
            w_line: new_w_line,
        }
    }

    fn conway_step(&self) -> Self {
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
            let mut new_state = buffer[idx].state;
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

        CellGrid { 
            mode: self.mode,
            cells: new_cells,
            width: self.width, 
            height: self.height,
            wrap: self.wrap,
            scroll: self.scroll,
            w_line: self.w_line,
        }
    }

    pub fn draw_cells(&self, ctx: &mut BTerm) {
        for c in &self.cells {
            ctx.set(c.x, c.y, c.fg, c.bg, c.glyph)
        }
    }
}