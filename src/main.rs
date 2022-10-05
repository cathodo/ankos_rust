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

#[derive(Default, Clone)]
pub struct Map {
    pub tiles : Vec<TileType>,
    pub width : i32,
    pub height : i32,
}

impl Map {
    pub fn xy_idx(x: i32, y: i32) -> usize {
        (y as usize * MAPWIDTH) + x as usize
    }
    
    fn new_map() -> Map {
        let mut map = Map{
            tiles: vec![TileType::Off; MAPCOUNT],
            width: MAPWIDTH as i32,
            height: MAPHEIGHT as i32,
        };
        // seed tile
        let seed_x = MAPWIDTH as i32/2;
        let seed_y  = MAPHEIGHT as i32/2;
        map.tiles[Self::xy_idx(seed_x, seed_y)] = TileType::On;
        
        map
    }

    fn ca_step(mut tiles: Vec<TileType>) -> Vec<TileType> {
    
        // loop over each position
        for x in 0..MAPWIDTH {
            for y in 0..MAPHEIGHT {
                // count surroundings
                let mut count = 0;
                for xc in 0..3 {
                    for yc in 0..3 {
                        if xc != 1 && yc != 1 {
                            let mut checkx: i32 = (x + xc - 1) as i32;
                            let mut checky: i32 = (y + yc - 1) as i32;
                            if checkx < 0 {checkx = MAPWIDTH as i32;}
                            if checkx > MAPWIDTH as i32 {checkx = 0;}
                            if checky < 0 {checky = MAPHEIGHT as i32;}
                            if checky > MAPHEIGHT as i32 {checky = 0;}
                            if tiles[Self::xy_idx(checkx, checky)] == TileType::On {
                                count += 1;
                            }
                        }
                    }
                }
    
                if count < 2 || count > 3 {
                    tiles[Self::xy_idx(x as i32, y as i32)] = TileType::Off
                } else {
                    tiles[Self::xy_idx(x as i32, y as i32)] = TileType::On
                }
    
    
            }
        }
    
        tiles
    }
}

struct State {
    ecs: World
}


fn draw_map(ecs: &World, ctx: &mut BTerm) {
    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;
    for tile in map.tiles.iter() {
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

        let mut map = self.ecs.fetch_mut::<Map>();

        map.tiles = Map::ca_step(map.tiles.clone());

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}



fn main() -> BError {
    env::set_var("RUST_BACKTRACE", "full");
    let context = BTermBuilder::simple80x50()
        .with_title("CA testing")
        .build()?;
    let mut gs = State {
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();

    let map: Map = Map::new_map();
    gs.ecs.insert(map);

    main_loop(context, gs)
}