extern crate piston_window;

use piston_window::*;

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

    while let Some(event) = window.next() {
        match event {
            Input::Press(Button::Keyboard(key)) => {
                match key {
                    Key::Up if max_branches < 16 => max_branches += 1,
                    Key::Down if max_branches > 1 => max_branches -= 1,
                    Key::Left => angle += 0.01,
                    Key::Right => angle -= 0.01,
                    _ => {}
                }
            },

            Input::Render(_) => {
                window.draw_2d(&event, |c, g| {
                    clear([0.92, 0.94, 0.98, 1.0], g);

                    for branch in branches(c.transform.trans(tree_position.0, tree_position.1), angle, 100.0, max_branches) {
                        line.radius(branch.1 / 10.0).draw([0.0, 0.0, 0.0, -branch.1], &c.draw_state, branch.0, g);
                    }
                });
            },
            _ => {}
        }
    }
}

fn branches(position: math::Matrix2d, angle: f64, depth: f64, max: u8) -> Vec<(math::Matrix2d, f64)> {
    let mut branch = Vec::new();

    branch.push((position, depth));

    if max > 0 {
        branch.extend(branches(position.trans(0.0, -depth).rot_rad( angle), angle, depth * 0.8, max - 1));
        branch.extend(branches(position.trans(0.0, -depth).rot_rad(-angle), angle, depth * 0.8, max - 1));
    }

    return branch;
}

