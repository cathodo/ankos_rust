use bracket_lib::prelude::{ RGB, GREEN, GREY, BLACK, to_cp437, BTerm };
use specs::prelude::*;
use super::{ World, Position, Renderable, Cells, };

pub fn draw_cell(ctx: &mut BTerm, x: i32, y: i32, state: u8) {
    let mut fg = RGB::named(BLACK);
    let bg = RGB::named(BLACK);
    let mut glyph = to_cp437('_');

    match state {
        0 => {
            fg = RGB::named(GREY);
            glyph = to_cp437('.');
        }
        1 => {
            fg = RGB::named(GREEN);
            glyph = to_cp437('#');
        }
        _ => {}
    }
    ctx.set(x, y, fg, bg, glyph);
}

pub fn draw_cells(cells: &Cells, ctx: &mut BTerm) {
    let mut x = 0;
    let mut y = 0;
    for c in cells.vec.iter() {
        draw_cell(ctx, x, y, c.state);
        // Move the coordinates
        x += 1;
        if x > cells.width as i32 - 1 {
            x = 0;
            y += 1;
        }
    }
}