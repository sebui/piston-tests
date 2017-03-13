extern crate piston_window;
extern crate rand;
extern crate nalgebra;

use piston_window::*;
use nalgebra::{Vector2, Point2,Isometry2};
use rand::Rng;

const WINDOW_SIZE: [u32; 2] = [1280, 720];

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Piston - Fractal Tree", WINDOW_SIZE)
        .exit_on_esc(true)
        .opengl(OpenGL::V3_2)
        .resizable(false)
        .build()
        .unwrap_or_else(|e| {
            panic!("Failed to build PistonWindow: {}", e)
        });

    window.set_ups(60);
    window.set_max_fps(60);

    let line = Line::new([0.2, 0.3, 0.6, 1.0], 1.0)
        .shape(line::Shape::Square);

    let mut max_branches = 8;
    let mut angle = ::std::f64::consts::FRAC_PI_4;

    let tree_position = (
        WINDOW_SIZE[0] as f64 / 2.0,
        WINDOW_SIZE[1] as f64 - 200.0
    );

    let mut particles: Vec<(Point2<f64>, Vector2<f64>)> = {
        let mut rng = rand::thread_rng();
        (0..100).map(|_| (rng.gen(), rng.gen()) ).collect()
    };

    let square = rectangle::square(0.0, 0.0, 10.0);

    while let Some(event) = window.next() {
        match event {

            Input::Update(arg) => {
                for particle in &mut particles {
                    particle.0 += particle.1;
                    particle.0 *= Isometry2::new(Vector2::new(arg.dt, arg.dt), ::std::f64::consts::PI);
                }
            }

            Input::Render(_) => {
                window.draw_2d(&event, |c, g| {
                    clear([0.92, 0.94, 0.98, 1.0], g);

                    for particle in &particles {
                        rectangle([1.0, 0.0, 0.0, 1.0], square, c.transform.trans(particle.0[0], particle.0[1]), g);
                    }
                });
            }
            _ => {}
        }
    }
}


