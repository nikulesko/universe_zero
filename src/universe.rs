use piston::input::*;
use opengl_graphics::GlGraphics;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub enum Flora {
    Mineral,
    Sunbeam,
    Organic,
    Empty,
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    flora: Flora,
    energy: u32,
    //color converter https://doc.instantreality.org/tools/color_calculator/
    color: [f32; 4],
}

impl Cell {
    fn new(c_flora: Flora) -> Cell {
        match c_flora {
            Flora::Sunbeam =>
                Cell{
                    flora: c_flora,
                    energy: 5,
                    color: [0.12, 0.51, 0.295, 1.0]
                },
            Flora::Mineral =>
                Cell{
                    flora: c_flora,
                    energy: 10,
                    color: [0.2, 0.59, 0.86, 1.0]
                },
            Flora::Organic =>
                Cell{
                    flora: c_flora,
                    energy: 10,
                    color: [0.9, 0.295, 0.235, 1.0]
                },
            Flora::Empty =>
                Cell{
                    flora: c_flora,
                    energy: 0,
                    color: [1.0, 1.0, 1.0, 1.0]
                },
        }
    }

    fn energy_increment(&mut self) {
        if self.energy > 0 {
            self.energy += 1;
        }
    }

    pub fn destroy(&mut self) {
        self.energy = 0;
    }

    pub fn get_energy(&mut self) -> u32 {
        self.energy
    }
}

#[derive(Clone, Debug)]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    pub fn new(w: u32, h: u32) -> Universe {

        let length = w * h;

        let field = (0..length)
            .map(|_| {

                let x = rand::thread_rng().gen::<u32>();

                if x % 3 == 0 {
                    Cell::new(Flora::Sunbeam)
                } else if x % 7 == 0  {
                    Cell::new(Flora::Organic)
                } else if x % 9 == 0  {
                    Cell::new(Flora::Mineral)
                } else {
                    Cell::new(Flora::Empty)
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

    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs, rectangle_x: u32, rectangle_y: u32, rectangle_size: u32) {

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];

                //cell.energy_increment();

                let square = graphics::rectangle::square(
                    (row * rectangle_x) as f64,
                    (col * rectangle_y) as f64,
                    rectangle_size as f64);

                gl.draw(args.viewport(), |c, gl| {
                    let transform = c.transform;
                    graphics::rectangle(cell.color, square, transform, gl);
                });
            }
        }
    }

    fn update(&mut self) {
        println!("ewe");
    }
}