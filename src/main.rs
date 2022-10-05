use bracket_lib::prelude::*;
use specs::prelude::*;
use std::env;
use specs_derive::Component;

pub const MAPWIDTH : usize = 80;
pub const MAPHEIGHT: usize = 50;
pub const MAPCOUNT: usize = MAPWIDTH * MAPHEIGHT;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32
}

#[derive(Component)]
struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    On, Off
}
struct State {
    ecs: World
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * MAPWIDTH as usize) + x as usize
}

pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Off; MAPCOUNT];
    // seed tile
    let pos = xy_idx(MAPWIDTH as i32/2, MAPHEIGHT as i32/2);
    map[pos] = TileType::On;
    
    map
}


fn draw_map(map: &[TileType], ctx: &mut BTerm) {
    let mut y = 0;
    let mut x = 0;
    for tile in map.iter() {
        // Render a tile depending upon the tile type
        match tile {
            TileType::Off => {
                ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), to_cp437('.'));
            }
            TileType::On => {
                ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), to_cp437('#'));
            }
        }
        
        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
    
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut BTerm) {
        ctx.cls();

        let mut map = self.ecs.fetch_mut::<Vec<TileType>>().to_vec();

        map = ca_step(map);
        
        draw_map(&map, ctx);
        
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn ca_step(map: Vec<TileType>) -> Vec<TileType> {
    let width = MAPWIDTH as i32;
    let height = MAPHEIGHT as i32;

    // buffer for next state
    let mut new_map = map.clone();
    // loop over all central cells
    for x in 0..width {
        for y in 0..height {
            let mut count = 0;
            // context cells
            for xmod in -1..1 {
                for ymod in -1..1 {
                    // skip central
                    if !(xmod == 0 && ymod == 0) {
                        let mut xpos = x+xmod;
                        let mut ypos = y+ymod;
                        // wrap cell context around the display positions 
                        if (xpos) > width { xpos=0+xmod }
                        if (xpos) < 0 { xpos=height+xmod }
                        if (ypos) > height { ypos=0+ymod }
                        if (xpos) < 0 { ypos=height+ymod }
                        if map[xy_idx(xpos, ypos)] == TileType::On { count +=1; }
                    }
                }
            }
            // modify central cell
            let pos = xy_idx(x, y);
            if count < 2 || count > 3 {
                new_map[pos] = TileType::Off;
            } else {
                new_map[pos] = TileType::On;
            }
        }
    }
    new_map
}


fn main() -> BError {
    env::set_var("RUST_BACKTRACE", "1");
    let mut context = BTermBuilder::simple80x50()
        .with_title("CA testing")
        .build()?;
    context.with_post_scanlines(false);
    let mut gs = State {
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();

    gs.ecs.insert(new_map());

    main_loop(context, gs)
}