use bracket_lib::prelude::{ RGB, to_cp437, BLACK, GRAY, GREEN, console };
use specs::prelude::*;
use specs_derive::*;
use super::{ Position, Renderable, World, SCREENWIDTH, SCREENHEIGHT, idx_xy };

#[derive(Component, Default, PartialEq, Clone, Copy)]
pub enum CellState {
    #[default]
    Off = 0,
    On = 1,
}

#[derive(Component)]
pub struct Moore {
    pub r: usize,
}

#[derive(Component)]
pub struct Cell {
    pub state: CellState,
    pub moore: Moore,
}

pub fn insert_cell_entities(ecs: &mut World) {
    // bg cells
    for idx in 0..SCREENHEIGHT*SCREENWIDTH {
        let (x, y) = idx_xy(idx);
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

    // seed cell
    ecs
        .create_entity()
        .with(Position { x: SCREENWIDTH as i32/2, y: SCREENHEIGHT as i32/2 })
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

pub fn change_cell_lifeness(render: &mut Renderable, cell: &mut Cell, live: bool) {
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

        for (_entity, pos, render, cell) in (&entities, &positions, &mut renderables, &mut cells).join() {
            if pos.x == 48 && pos.y == 24 {
                change_cell_lifeness(render, cell, cell.state == CellState::Off);
            }
        }
    }
}