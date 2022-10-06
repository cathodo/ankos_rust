use bracket_lib::prelude::*;
use specs::prelude::*;
mod components;
pub use components::*;
mod map;
pub use map::*;
mod automata_system;
use automata_system::*;

pub struct State {
    pub ecs: World
}

impl State {
    fn run_systems(&mut self) {
        let mut ca = AutomataSystem{};
        ca.run_now(&self.ecs);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut BTerm) {
        ctx.cls();

        self.run_systems();

        draw_map(&self.ecs, ctx);
        
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
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();

    let map : Map = Map::new();
    gs.ecs.insert(map);

    main_loop(context, gs)
}