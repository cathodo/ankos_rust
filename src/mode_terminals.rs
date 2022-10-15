use bracket_lib::prelude::*;
use specs::prelude::*;

use super::{ WRAPCELLS, PERCENTRANDOMSEED, SCROLLMODE, CellGrid, Mode };

pub fn setup_ecs_2d(ecs: &mut World, width: usize, height: usize) {
    // CA
    ecs.register::<CellGrid>();

    let w: i32 = width as i32;
    let h: i32 = height as i32;
    // randomly generate seeds
    // set at about 11 percent of screen area atm
    let nseeds = w*h/100*PERCENTRANDOMSEED;
    let mut rng = RandomNumberGenerator::new();
    let mut seeds: Vec<(i32, i32)> = Vec::new();
    for _s in 0..nseeds {
        seeds.push((w-rng.range(0, w), h-rng.range(0, h)));
    }

    let cells = CellGrid::new(Mode::Conway, width, height, seeds, WRAPCELLS, SCROLLMODE);
    ecs.insert(cells);
}

pub fn setup_ecs_1d(ecs: &mut World, width: usize, height: usize) {
    // CA
    ecs.register::<CellGrid>();

    let w: i32 = width as i32;
    let _h: i32 = height as i32;
    // init seed is center first row
    let mut seeds: Vec<(i32, i32)> = Vec::new();
    seeds.push((w/2,0));

    let cells = CellGrid::new(Mode::Wolfram, width, height, seeds, WRAPCELLS, SCROLLMODE);
    ecs.insert(cells);
}