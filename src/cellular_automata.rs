use specs::*;
use specs_derive::*;

#[derive(Component)]
pub enum Currency {
    Now = 0,
    Old = 1,
}

#[derive(Default, PartialEq, Clone, Copy)]
pub struct Cell {
    pub state: u8,
}

pub struct Cells {
    pub width: usize,
    pub height: usize,
    pub vec: Vec<Cell>,
}

impl Cells {
    pub fn new(w: usize, h: usize) -> Self {
        let mut v = Vec::new();
        v.reserve_exact(w * h);
        for _i in 0..w * h {
            v.push(Cell::default());
        }
        Cells {
            width: w,
            height: h,
            vec: v,
        }
    }

    pub fn read(&self, i: usize) -> &Cell {
        &self.vec[i]
    }
    pub fn write(&mut self, i: usize, cell: Cell) {
        self.vec[i] = cell;
    }
    pub fn read_pos(&self, x: usize, y: usize) -> &Cell {
        self.read(x + self.width * y)
    }
    pub fn write_pos(&mut self, x: usize, y: usize, cell: Cell)
    {
        self.write(x + self.width * y, cell)
    }

    pub fn step(&self) -> Self
    {
        let moore = vec![
            (-1isize, -1isize),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        Cells{
            width: self.width,
            height: self.height,
            vec:
            (0..self.vec.len())
                .map(|i| {
                    let x0 = i % self.width;
                    let y0 = i / self.width;
                    moore.iter().filter_map(|&(v_x, v_y)| {
                        let x = x0 as isize + v_x;
                        let y = y0 as isize + v_y;
                        if x < 0 || y < 0 || (x as usize) >= self.width || y as usize >= self.height {
                            None
                        } else {
                            Some(self.vec[(x as usize) + self.width * (y as usize)].state)
                        }
                    }).fold(0, |somme, ngh| somme+ngh)
                }).zip(0..self.vec.len()).map(
                    |(sum, i)| Cell{state:
                                    match sum
                                    {
                                        2 => self.vec[i].state,
                                        3 => 1,
                                        _ => 0
                                    }}
                )
                .collect()
        }
    }

    pub fn evolve(&self, n: usize)
    {
        match n
        {
            0 => (),
            _ => self.step().evolve(n-1)
        }
    }
}