use bracket_lib::prelude::*;
use specs::prelude::*;
use specs_derive::Component;
mod cell_printer;
use cell_printer::*;
mod cellular_automata;
use cellular_automata::*;

const SCREENWIDTH: usize = 80;
const SCREENHEIGHT: usize = 50;

#[derive(Component)]
struct Position {
    x: i32, 
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

struct State {
    ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut BTerm) {
        ctx.cls();

        match ctx.key {
            None => {}
            Some(key) => match key {
                VirtualKeyCode::Space => { self.ecs.fetch::<Cells>().step(); }
                _ => {}
            }
        }

        let cells = self.ecs.fetch::<Cells>();
        draw_cells(&cells, ctx);
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
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Currency>();

    let mut cells = Cells::new(SCREENWIDTH, SCREENHEIGHT);
    cells.write_pos(SCREENWIDTH as usize/2, SCREENHEIGHT as usize/2, Cell{ state: 1 as u8 });
    gs.ecs.insert(cells);

    main_loop(context, gs)
}