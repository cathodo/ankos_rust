use bracket_lib::prelude::*;
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;



struct State {}
impl GameState for State {
    fn tick(&mut self, ctx : &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello Rust World");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let gs = State{ };
    main_loop(context, gs)
}