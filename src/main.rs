use std::env;

use bracket_lib::prelude::*;
use specs::prelude::*;
mod cellular_automata;
use cellular_automata::*;
mod mode_terminals;
use mode_terminals::*;

pub const SCREENWIDTH: usize = 160;
pub const SCREENHEIGHT: usize = 120;
const CYCLESPERSECOND: f32 = 10.0;
// 2d params
pub const WRAPCELLS: bool = false;
const PERCENTRANDOMSEED: i32 = 11;
// 1d params
const SCROLLDOWN: bool = true;
// choose 2d or 1d
pub const MODE: Mode = Mode::Wolfram;

////////////////////////////////////////
////////////////////////////////////////
////////////////////////////////////////

#[derive(PartialEq, Copy, Clone)]
pub enum Mode { Conway, Wolfram }

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
        
        let mut cells = self.ecs.fetch_mut::<CellGrid>().to_owned();
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
    env::set_var("RUST_BACKTRACE", "1");
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
        _ => {}, 
    }

    main_loop(context, gs)
}