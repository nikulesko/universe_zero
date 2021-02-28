mod universe;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;
use piston::EventLoop;
use crate::universe::Universe;

struct Game {
    gl: GlGraphics,
    universe: Universe,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear([1.0, 1.0, 1.0, 1.0], gl);
        });

        let rectangle_x = 5;
        let rectangle_y = 5;
        let rectangle_size = 5;

        self.universe.render(&mut self.gl, arg, rectangle_x, rectangle_y, rectangle_size);
    }

/*    fn update(&mut self) {
        println!("update")
    }*/
}

fn main() {
    let opengl = OpenGL::V3_3;

    let rectangle_x = 5;
    let rectangle_y = 5;

    let universe_width = 200; //ширина вселенной
    let universe_height = 200; //длина вселенной

    let mut window: GlutinWindow = WindowSettings::new(
        "Вселенная 0",
        [universe_height * rectangle_x, universe_width * rectangle_y])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        universe: Universe::new(universe_width, universe_height)
    };

    let mut events = Events::new(EventSettings::new()).ups(30);

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }
    }
}
