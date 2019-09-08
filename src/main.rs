extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate touch_visualizer;

#[cfg(feature = "include_glfw")]
extern crate glfw_window;
#[cfg(feature = "include_glutin")]
extern crate glutin_window;
#[cfg(feature = "include_sdl2")]
extern crate sdl2_window;

#[cfg(feature = "include_glfw")]
use glfw_window::GlfwWindow as AppWindow;
#[cfg(feature = "include_glutin")]
use glutin_window::GlutinWindow as AppWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
#[cfg(feature = "include_sdl2")]
use sdl2_window::Sdl2Window as AppWindow;
mod models;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    x_ship_movement: f64,
    y_ship_movement: f64, // TODO create ship struct
    beams: Vec<models::beam::Beam>,
    rocks: Vec<models::rock::Rock>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GRAYISH: [f32; 4] = [0.3, 0.35, 0.31, 1.0];

        let rotation = self.rotation;
        let x_ship_movement = self.x_ship_movement;
        let y_ship_movement = self.y_ship_movement;
        let beams = &self.beams;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GRAYISH, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-x_ship_movement, -y_ship_movement);

            // SHIP
            let ship = &[
                [0.0, 0.0],
                [40.0, 0.0],
                [30.0, -10.0],
                [30.0, -20.0],
                [20.0, -30.0],
                [10.0, -20.0],
                [10.0, -10.0],
                [0.0, 0.0],
            ];

            const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
            const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

            let mut i = 0;

            while i <= ship.len() - 2 {
                let mut cc = RED;
                if i < 2 || i > 5 {
                    cc = GREEN
                }

                line(
                    cc,
                    2.0,
                    [ship[i][0], ship[i][1], ship[i + 1][0], ship[i + 1][1]],
                    transform,
                    gl,
                );
                i = i + 1;
            }

            //BEAM
            for beam in beams {
                let beam_transform = c
                    .transform
                    .trans(x, y)
                    .rot_rad(rotation)
                    .trans(beam.x, -beam.y);

                let square = rectangle::square(20.0, -30.0, 5.0);
                rectangle(RED, square, beam_transform, gl);
            }
        });
    }

    fn update_movement_x(&mut self, _move: f64) {
        // Rotate 2 radians per second.
        self.x_ship_movement += _move;
    }

    fn update_movement_y(&mut self, _move: f64) {
        // Rotate 2 radians per second.
        self.y_ship_movement += _move;
    }

    fn update(&mut self, args: &UpdateArgs) {
        for beam in &mut self.beams {
            beam.y += 50.0 * args.dt;
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    let mut window: AppWindow = WindowSettings::new("piston-example-user_input", [400, 400])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        x_ship_movement: 0.0,
        y_ship_movement: 0.0,
        beams: Vec::new(),
        rocks: Vec::new()
    };

    let mut _move: f64 = 0.0;

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Up {
                app.update_movement_y(2.5);
            }

            if key == Key::Down {
                app.update_movement_y(-2.5);
            }

            if key == Key::Left {
                app.update_movement_x(2.5);
            }

            if key == Key::Right {
                app.update_movement_x(-2.5);
            }

            if key == Key::Space {
                app.beams.push(models::beam::Beam {
                    x: -app.x_ship_movement,
                    y: app.y_ship_movement,
                });
            }
        };
    }
}
