use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL};

use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tp {
    Dead = 0,
    Alive = 1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    tp: Tp,
    age: u32,
}

impl Cell {
    fn new(t: Tp) -> Cell {
        let mut rng = rand::thread_rng();

        Cell {
            age: rng.gen::<u8>() as u32,
            tp: t
        }
    }

    fn age_decrement(&mut self) {
        if self.age > 0 {
            self.age -= 1;
        }
    }
}

pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    pub(crate) fn new(w: u32, h: u32) -> Universe {

        let field = (0..w * h)
            .map(|i| {
                if i % 2 == 0 {
                    Cell::new(Tp::Alive)
                } else {
                    Cell::new(Tp::Dead)
                }
            })
            .collect();

        Universe{
            width: w,
            height: h,
            cells: field
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub(crate) fn render(&self, gl: &mut GlGraphics, args: &RenderArgs, rectangle_x: u32, rectangle_y: u32, rectangle_size: u32) {

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let mut cell = self.cells[idx];

                cell.age_decrement();

                let square = graphics::rectangle::square((row * rectangle_x) as f64, (col * rectangle_y) as f64, rectangle_size as f64);

                match cell.tp {
                    Tp::Alive =>
                        gl.draw(args.viewport(), |c, gl| {
                            let transform = c.transform;

                            graphics::rectangle([125.0, 0.0, 0.0, 1.0], square, transform, gl);
                        }),
                    Tp::Dead =>
                        gl.draw(args.viewport(), |c, gl| {
                            let transform = c.transform;

                            graphics::rectangle([0.0, 125.0, 0.0, 1.0], square, transform, gl);
                        }),
                }
            }
        }
    }

    fn update(&mut self) {
        println!("ewe");
    }
}