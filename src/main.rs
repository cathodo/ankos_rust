use bracket_lib::prelude::*;
use specs::prelude::*;

#[derive(PartialEq, Clone, Copy)]
enum CellState {
    Off = 0,
    On = 1, 
}

const RUNFREQUENCY: i32 = 6;
const SCREENWIDTH: usize = 80;
const SCREENHEIGHT: usize = 50;
const SCREENCOUNT: usize = SCREENWIDTH*SCREENHEIGHT;
// with layout [nw, n, ne, w, c, e, sw, s, se]
const NEIGHBORHOOD: [(i32, i32); 9] = [
    (-1,  1),
    ( 0,  1),
    ( 1,  1),
    (-1,  0),
    ( 0,  0),
    ( 1,  0),
    (-1, -1),
    ( 0, -1),
    ( 1, -1)
];

fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * SCREENWIDTH as usize) + x as usize
}

fn idx_xy(idx: usize) -> (i32, i32) {
    let x = (idx % SCREENWIDTH) as i32;
    let y = (idx as f32/SCREENWIDTH as f32).floor() as i32;
    return (x, y)
}

fn moor_xy_2d(mut x: i32, mut y: i32) -> (i32, i32) {
    let width = SCREENWIDTH as i32-1;
    let height = SCREENHEIGHT as i32-1;

    if x > width {
        x -= width;
    }
    if x < 0 {
        x += width;
    }
    if y > height {
        y -= height;
    }
    if y < 0 {
        y += height;
    }

    (x, y)
}

fn spawn_initial_state() -> Vec<CellState> {
    // all off
    let mut state = vec![CellState::Off; SCREENCOUNT];

    // add seed 
    let idx = xy_idx(SCREENWIDTH as i32/2, SCREENHEIGHT as i32/2);
    state[idx] = CellState::On;

    state
}

fn ca_step(cells: Vec<CellState>, run: bool) -> Vec<CellState> {
    let mut new_cells: Vec<CellState> = Vec::new();
    if run {
        let mut neighbour_count: i8 = 0;
        let mut y = 0;
        let mut x = 0;
        for _idx in 0..SCREENCOUNT {
            // count neighbours
            for (moorx, moory) in NEIGHBORHOOD.iter() {
                let (mx, my) = moor_xy_2d( x + moorx, y + moory );
                let midx = xy_idx(mx, my);
                if cells[midx] == CellState::On {
                    neighbour_count += 1;
                }
                
            }
            // set cell type
            //if neighbour_count < 2 || neighbour_count > 3 {
            if neighbour_count < 1 {
                new_cells.push(CellState::Off);
            } else {
                new_cells.push(CellState::On);
            }
    
            // Move the coordinates
            x += 1;
            if x > SCREENWIDTH as i32-1 {
                x = 0;
                y += 1;
            }
        }
    } else {
        new_cells = cells;
    }
    new_cells
}

fn draw_cells(cells: &[CellState], ctx : &mut BTerm) {
    let mut y = 0;
    let mut x = 0;
    for cell in cells.iter() {
        // Render a tile depending upon the tile type
        let fg;
        let bg = RGB::from_f32(0., 0., 0.);
        let glyph;
        match cell {
            CellState::Off => {
                fg = RGB::named(GREY);
                glyph = to_cp437('.');
            }
            CellState::On => {
                fg = RGB::named(GREEN);
                glyph = to_cp437('#');
            }
        }
        ctx.set(x, y, fg, bg, glyph);

        // Move the coordinates
        x += 1;
        if x > SCREENWIDTH as i32-1 {
            x = 0;
            y += 1;
        }
    }
}

struct State {
    ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut BTerm) {
        ctx.cls();
        
        let cells = self.ecs.fetch::<Vec<CellState>>();
        let mut compute = false;
        // ca step
        match ctx.key {
            None => {}
            Some(key) => match key {
                VirtualKeyCode::Space => { compute = true; }
                _ => {}
            }
        }
        // ca step
        let new_cells = ca_step(cells.to_vec(), compute);

        draw_cells(&new_cells, ctx);
    }
}

fn main() -> BError {
    let mut context = BTermBuilder::simple80x50()
        .with_title("CA testing")
        .build()?;
    context.with_post_scanlines(false);
    let mut gs = State {
        ecs: World::new()
    };
    gs.ecs.insert(spawn_initial_state());

    main_loop(context, gs)
}