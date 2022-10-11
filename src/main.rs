use bracket_lib::prelude::*;
use specs::prelude::*;
mod cellular_automata;
use cellular_automata::*;
mod mode_terminals;
use mode_terminals::*;

pub const SCREENWIDTH: usize = 160;
pub const SCREENHEIGHT: usize = 100;
const CYCLESPERSECOND: f32 = 30.0;
////// choose 2d or 1d
pub const MODE: Mode = Mode::Conway;

////// use space to pause/unpause (false), or just to advance one state (true)
pub const SPACEONESTEP: bool = false;
// if cells on edge wrap around to check neighbours
// slightly diff behaviour on 2d vs 1d, 2d wraps all edges, 
// 1d only wraps x axis edges (L & R not top & bottom) see SCROLLMODE
pub const WRAPCELLS: bool = true;

//// 2d params
// percentage of screen to be random cells in seed init
const PERCENTRANDOMSEED: i32 = 11;

//// 1d params
// shift whole thing, loop around to top, or don't do either
const SCROLLMODE: ScrollMode = ScrollMode::Shift;
// can be 0-256
const RULE: i32 = 30;

////////////////////////////////////////
////////////////////////////////////////
////////////////////////////////////////

#[derive(PartialEq, Copy, Clone)]
pub enum Mode { Conway, Wolfram }

#[derive(PartialEq, Copy, Clone)]
pub enum ScrollMode { Shift, Loop, Stop }

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

            _ => { current_state }
        },
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut BTerm) {
        ctx.cls();
        
        let mut cells = self.ecs.fetch_mut::<CellGrid>().to_owned();
        cells.draw_cells(ctx);
        
        match self.runstate {
            RunState::Running => {
                cells = cells.step();
                if SPACEONESTEP { self.runstate = RunState::Paused; }
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

    match MODE {
        Mode::Conway => { setup_ecs_2d(&mut gs.ecs, SCREENWIDTH, SCREENHEIGHT) },
        Mode::Wolfram => { setup_ecs_1d(&mut gs.ecs, SCREENWIDTH, SCREENHEIGHT) },
    }

    main_loop(context, gs)
}