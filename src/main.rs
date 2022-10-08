use bracket_lib::prelude::*;
use specs::prelude::*;
mod cellular_automata;
use cellular_automata::*;

pub const SCREENWIDTH: usize = 80;
pub const SCREENHEIGHT: usize = 60;
pub const WRAPCELLS: bool = true;


#[derive(PartialEq, Copy, Clone)]
pub enum RunState { Paused, Running }

struct State {
    ecs: World,
    pub runstate : RunState
}

fn player_input(ctx: &mut BTerm) -> RunState {
    // Player movement
    match ctx.key {
        None => { return RunState::Paused } // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Space => { return RunState::Running }

            _ => { return RunState::Paused }
        },
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut BTerm) {
        ctx.cls();
        
        let mut cells: CellGrid2d = self.ecs.fetch_mut::<CellGrid2d>().to_owned();
        cells.draw_cells(ctx);
        
        match self.runstate {
            RunState::Running => {
                cells = cells.step();
                self.runstate = RunState::Running;
            }
            RunState::Paused => {
                self.runstate = player_input(ctx);
            }
        }
        
        self.ecs.insert(cells);
    }
}

fn main() -> BError {
    let mut context = BTermBuilder::simple(SCREENWIDTH, SCREENHEIGHT)
        .unwrap()
        .with_title("CA testing")
        .build()?;
    context.with_post_scanlines(true);


    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Paused,
    };
    gs.ecs.register::<CellGrid2d>();

    let w: i32 = SCREENWIDTH as i32;
    let h: i32 = SCREENHEIGHT as i32;

    let seeds: Vec<(i32, i32)> = vec![
        (w/2, h/2),
        (w/2-1, h/2),
        (w/2, h/2-1),
    ];

    let cells = CellGrid2d::new(SCREENWIDTH, SCREENHEIGHT, seeds, WRAPCELLS);

    gs.ecs.insert(cells);

    main_loop(context, gs)
}