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
        (0..2000).map(|_| (
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

    let square = rectangle::centered_square(0.0, 0.0, 4.0);
    let line = Line::new([0.5, 0.5, 0.5, 1.0], 1.0)
        .shape(line::Shape::Square);

    let mut cursor_pos = Point2::new(0.0, 0.0);
    let mut lines: Vec<(Point2<f64>, Point2<f64>)> = Vec::new();


    while let Some(event) = window.next() {
        match event {
            Input::Move(Motion::MouseCursor(x, y)) => {
                cursor_pos.x = x;
                cursor_pos.y = y;
            }

            Input::Update(args) => {
                lines.clear();

                for x in 0..particles.len()  {
                    particles[x].0 += particles[x].1 * args.dt;
                    particles[x].1 *= 0.9;

                    if nalgebra::distance(&particles[x].0, &cursor_pos) < 150.0 {
                        particles[x].1 += particles[x].0 - cursor_pos;
                    }

                    if particles[x].0.x < 0.0 {
                        particles[x].0.x == 0.0;
                        particles[x].1.x = -particles[x].1.x;
                    }
                    if particles[x].0.x > WINDOW_SIZE[0] as f64 {
                        particles[x].0.x == WINDOW_SIZE[0] as f64;
                        particles[x].1.x = -particles[x].1.x;
                    }
                    if particles[x].0.y < 0.0 {
                        particles[x].0.y == 0.0;
                        particles[x].1.y = -particles[x].1.y;
                    }
                    if particles[x].0.y > WINDOW_SIZE[1] as f64 {
                        particles[x].0.y == WINDOW_SIZE[1] as f64;
                        particles[x].1.y = -particles[x].1.y;
                    }

                    for y in 0..particles.len() {
                        if nalgebra::distance(&particles[x].0, &particles[y].0) < 20.0 {
                            particles[x].1 += particles[x].0 - particles[y].0;
                            lines.push((particles[x].0.clone(), particles[y].0.clone()));
                        }
                    }
                }

            }

            Input::Render(_) => {
                window.draw_2d(&event, |c, g| {
                    clear([0.92, 0.94, 0.98, 1.0], g);

                    for l in &lines {
                        line.draw([l.0.x, l.0.y, l.1.x, l.1.y], &c.draw_state, c.transform, g);
                    }

                    for particle in &particles {
                        rectangle([0.0, 0.0, 0.0, 1.0], square, c.transform.trans(particle.0.x, particle.0.y), g);
                    }
                });
            }
            _ => {}
        }
    }
}


