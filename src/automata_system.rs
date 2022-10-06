use specs::prelude::*;
use super::{ CellState, Map, Position };

pub struct AutomataSystem {}

impl<'a> System<'a> for AutomataSystem{
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Position>,
        //Cells<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, position) = data;
        //let idx = map.xy_idx(position.x, position.y);
    }

}

// fn ca_step(map: Vec<CellState>) -> Vec<CellState> {
//     let width = MAPWIDTH as i32;
//     let height = MAPHEIGHT as i32;

//     // buffer for next state
//     let mut new_map = map.clone();
//     // loop over all central cells
//     for x in 0..width {
//         for y in 0..height {
//             let mut count = 0;
//             // context cells
//             for xmod in -1..1 {
//                 for ymod in -1..1 {
//                     // skip central
//                     if !(xmod == 0 && ymod == 0) {
//                         let mut xpos = x+xmod;
//                         let mut ypos = y+ymod;
//                         // wrap cell context around the display positions 
//                         if (xpos) > width { xpos=0+xmod }
//                         if (xpos) < 0 { xpos=height+xmod }
//                         if (ypos) > height { ypos=0+ymod }
//                         if (xpos) < 0 { ypos=height+ymod }
//                         if map[xy_idx(xpos, ypos)] == CellState::On { count +=1; }
//                     }
//                 }
//             }
//             // modify central cell
//             let pos = xy_idx(x, y);
//             if count < 2 || count > 3 {
//                 new_map[pos] = CellState::Off;
//             } else {
//                 new_map[pos] = CellState::On;
//             }
//         }
//     }
//     new_map
// }