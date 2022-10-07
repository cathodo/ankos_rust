use bracket_lib::prelude::{ RGB, to_cp437, BLACK, GRAY, GREEN, console };
use specs::prelude::*;
use specs_derive::*;
use super::{ Position, Renderable, World, SCREENWIDTH, SCREENHEIGHT, idx_xy };

pub const WRAPCELLS: bool = true;

#[derive(Component, Default, PartialEq, Clone, Copy)]
pub enum CellState {
    #[default]
    Off = 0,
    On = 1,
}

#[derive(Component, Clone, Copy)]
pub struct Moore {
    pub r: i32,
}

#[derive(Component, Clone, Copy)]
pub struct Cell {
    pub state: CellState,
    pub moore: Moore,
}

fn insert_live_cell_entity(ecs: &mut World, x: i32, y: i32) {
    ecs
        .create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: to_cp437('#'),
            fg: RGB::named(GREEN),
            bg: RGB::named(BLACK),
        })
        .with(Cell{
            state: CellState::On,
            moore: Moore { r: 1 },
        })
        .build();
}

fn insert_dead_cell_entity(ecs: &mut World, x: i32, y: i32) {
    ecs
    .create_entity()
    .with(Position { x, y })
    .with(Renderable {
        glyph: to_cp437('.'),
        fg: RGB::named(GRAY),
        bg: RGB::named(BLACK),
    })
    .with(Cell{
        state: CellState::Off,
        moore: Moore { r: 1 },
    })
    .build();
}

pub fn insert_cell_entities(ecs: &mut World) {
    // seed cells, need 3?
    let seeds: Vec<(i32, i32)> = vec![
        (SCREENWIDTH as i32/2, SCREENHEIGHT as i32/2),
        (SCREENWIDTH as i32/2 - 1, SCREENHEIGHT as i32/2),
        (SCREENWIDTH as i32/2, SCREENHEIGHT as i32/2 - 1),
    ];
    // bg cells
    for idx in 0..SCREENHEIGHT*SCREENWIDTH {
        let (x, y) = idx_xy(idx);
        if seeds.contains(&(x, y)){
            insert_live_cell_entity(ecs, x, y);
        } else {
            insert_dead_cell_entity(ecs, x, y);
        }
    }
}

pub fn change_cell_state(render: &mut Renderable, cell: &mut Cell, live: bool) {
    if live {
        render.fg = RGB::named(GREEN);
        render.bg = RGB::named(BLACK);
        render.glyph = to_cp437('#');
        cell.state = CellState::On;
    } else {
        render.fg = RGB::named(GRAY);
        render.bg = RGB::named(BLACK);
        render.glyph = to_cp437('.');
        cell.state = CellState::Off;
    }
}

pub fn get_moore_neighbour_counts(buffer: &Vec<(Position, Renderable, Cell)>, pos: &Position, moorad: i32) -> usize {
    
    let swidth = SCREENWIDTH as i32;
    let sheight = SCREENHEIGHT as i32;
    
    let moorxy: Vec<(i32, i32)> = vec![
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

    let mut count: usize = 0;

    let mut neighbourhood: Vec<(i32, i32)> = Vec::new();
    for _i in 1..moorad+1 {
        for &(x, y) in moorxy.iter(){
            neighbourhood.push((x+moorad, y+moorad));
        }
    }

    for &m in neighbourhood.iter() {
        // position
        let (mx, my) = m;
        let (mut new_x, mut new_y) = (pos.x + mx, pos.y + my);
        // adjust OOB positions (wrap around)
        if WRAPCELLS {
            if new_x < 0 { new_x += swidth };
            if new_x > swidth { new_x -= swidth };
            if new_y < 0 { new_y += sheight };
            if new_y > sheight { new_y -= sheight };
        }
        // searching buffer, should only have 1 hit
        let index_raw = buffer.iter().position(|&(p, _r, _c)| p.x == new_x && p.y == new_y);
        // shouldn't be none if we wrap?
        if !index_raw.is_none() {
            let index = index_raw.unwrap();
            let (_mp, _mr, mc) = buffer[index];
            if mc.state == CellState::On { count += 1 }
        }
        // check cell
    }

    count
}

pub struct AutomataStep {}

impl<'a> System<'a> for AutomataStep {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, Cell>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, positions, mut renderables, mut cells) = data;

        let mut buffer: Vec<(Position, Renderable, Cell)> = Vec::new();

        // load buffer
        for (_entity, pos, render, cell) in (&entities, &positions, &mut renderables, &mut cells).join() {
            buffer.push((*pos, *render, *cell));
        }

        // eval new cellstates
        for (_entity, pos, render, cell) in (&entities, &positions, &mut renderables, &mut cells).join() {
            // get moore neighbourhood from buffer
            let neighbours = get_moore_neighbour_counts(&buffer, pos, cell.moore.r);

            // determine new state 
            if cell.state == CellState::Off {
                if neighbours == 3 { change_cell_state(render, cell, true) }

            }
            if cell.state == CellState::On {
                if neighbours < 2 { change_cell_state(render, cell, false) }
                if neighbours > 3 { change_cell_state(render, cell, false) }
            }

        }
    }
}