use bracket_lib::prelude::*;
use specs::prelude::*;
use specs_derive::Component;
mod cellular_automata;
use cellular_automata::*;

pub const SCREENWIDTH: usize = 80;
pub const SCREENHEIGHT: usize = 50;

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * SCREENWIDTH) + x as usize
}

pub fn idx_xy(idx: usize) -> (i32, i32) {
    let x = idx % SCREENWIDTH;
    let y = (idx as f32 / SCREENWIDTH as f32).floor();
    return (x as i32, y as i32)
}

#[derive(Component, Clone, Copy)]
pub struct Position {
    pub x: i32, 
    pub y: i32,
}

#[derive(Component, Clone, Copy)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { Paused, Running }

struct State {
    ecs: World,
    pub runstate : RunState
}

impl State {
    fn run_systems(&mut self) {
        let mut ca = AutomataStep{};
        ca.run_now(&self.ecs);
        self.ecs.maintain();
    }
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

        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(ctx);
        }

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> BError {
    let mut context = BTermBuilder::simple80x50()
        .with_title("CA testing")
        .build()?;
    context.with_post_scanlines(false);

    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Paused,
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Cell>();
    gs.ecs.register::<Moore>();

    insert_cell_entities(&mut gs.ecs);

    main_loop(context, gs)
}