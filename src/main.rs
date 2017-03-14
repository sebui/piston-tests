extern crate piston_window;
extern crate rand;
extern crate nalgebra;

use piston_window::*;
use nalgebra::{Vector2, Point2};
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

    let mut particles: Vec<(Point2<f64>, Vector2<f64>)> = {
        let mut rng = rand::thread_rng();
        (0..1000).map(|_| (
                Point2::new(
                    rng.gen_range(0.0, WINDOW_SIZE[0] as f64),
                    rng.gen_range(0.0, WINDOW_SIZE[1] as f64)
                ),
                Vector2::new(
                    rng.gen_range(-50.0, 50.0),
                    rng.gen_range(-50.0, 50.0)
                )
            )).collect()
    };

    let square = rectangle::square(0.0, 0.0, 4.0);

    while let Some(event) = window.next() {
        match event {

            Input::Update(arg) => {
                for particle in &mut particles {
                    particle.0 += particle.1 * arg.dt;

                    if particle.0.x < 0.0 {
                        particle.0.x == 0.0;
                        particle.1.x = -particle.1.x;
                    }
                    if particle.0.x > WINDOW_SIZE[0] as f64 {
                        particle.0.x == WINDOW_SIZE[0] as f64;
                        particle.1.x = -particle.1.x;
                    }
                    if particle.0.y < 0.0 {
                        particle.0.y == 0.0;
                        particle.1.y = -particle.1.y;
                    }
                    if particle.0.y > WINDOW_SIZE[1] as f64 {
                        particle.0.y == WINDOW_SIZE[1] as f64;
                        particle.1.y = -particle.1.y;
                    }
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


