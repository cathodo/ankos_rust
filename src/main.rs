use bracket_lib::prelude::*;
use specs::prelude::*;
mod cellular_automata;
use cellular_automata::*;

pub const SCREENWIDTH: usize = 160;
pub const SCREENHEIGHT: usize = 120;
pub const WRAPCELLS: bool = false;
const CYCLESPERSECOND: f32 = 10.0;
const PERCENTRANDOMSEED: i32 = 11;


#[derive(PartialEq, Copy, Clone)]
pub enum RunState { Paused, Running }

struct State {
    ecs: World,
    pub runstate : RunState
}

fn player_input(current_state: RunState, ctx: &mut BTerm) -> RunState {
    // Player movement
    match ctx.key {
        None => { return current_state } // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Space => { 
                match current_state {
                    RunState::Running => { return RunState::Paused }
                    RunState::Paused => { return RunState::Running }
                }
             }

            _ => { return current_state }
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
                self.runstate = player_input(self.runstate, ctx);
            }
            RunState::Paused => {
                self.runstate = player_input(self.runstate, ctx);
            }
        }
        
        self.ecs.insert(cells);
    }
}

fn main() -> BError {
    let mut context = BTermBuilder::simple(SCREENWIDTH, SCREENHEIGHT)
        .unwrap()
        .with_title("CA testing")
        .with_fps_cap(CYCLESPERSECOND)
        .build()?;
    context.with_post_scanlines(true);


    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Paused,
    };
    gs.ecs.register::<CellGrid2d>();

    let w: i32 = SCREENWIDTH as i32;
    let h: i32 = SCREENHEIGHT as i32;

    // randomly generate seeds
    // set at about 11 percent of screen area atm
    let nseeds = w*h/100*PERCENTRANDOMSEED;
    let mut rng = RandomNumberGenerator::new();
    let mut seeds: Vec<(i32, i32)> = Vec::new();
    for _s in 0..nseeds {
        seeds.push((w-rng.range(0, w), h-rng.range(0, h)));
    }

    let cells = CellGrid2d::new(SCREENWIDTH, SCREENHEIGHT, seeds, WRAPCELLS);
    gs.ecs.insert(cells);

    main_loop(context, gs)
}