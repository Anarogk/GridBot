extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use rand::Rng;

const GRID_SIZE: usize = 20;
const CELL_SIZE: f64 = 20.0;

struct Robot {
    x: usize,
    y: usize,
}

impl Robot {
    fn new(x: usize, y: usize) -> Self {
        Robot { x, y }
    }

    fn move_robot(&mut self, dx: isize, dy: isize) {
        self.x = ((self.x as isize + dx).max(0).min(GRID_SIZE as isize - 1)) as usize;
        self.y = ((self.y as isize + dy).max(0).min(GRID_SIZE as isize - 1)) as usize;
    }
}

struct World {
    robot: Robot,
    obstacles: Vec<(usize, usize)>,
}

impl World {
    fn new() -> Self {
        let robot = Robot::new(GRID_SIZE / 2, GRID_SIZE / 2);
        let obstacles = World::generate_obstacles(10);

        World { robot, obstacles }
    }

    fn generate_obstacles(count: usize) -> Vec<(usize, usize)> {
        let mut rng = rand::thread_rng();
        let mut obstacles = Vec::new();

        for _ in 0..count {
            let x = rng.gen_range(0..GRID_SIZE);
            let y = rng.gen_range(0..GRID_SIZE);
            obstacles.push((x, y));
        }

        obstacles
    }

    fn is_obstacle(&self, x: usize, y: usize) -> bool {
        self.obstacles.contains(&(x, y))
    }

    fn move_robot(&mut self, dx: isize, dy: isize) {
        let new_x = (self.robot.x as isize + dx).max(0).min(GRID_SIZE as isize - 1) as usize;
        let new_y = (self.robot.y as isize + dy).max(0).min(GRID_SIZE as isize - 1) as usize;

        if !self.is_obstacle(new_x, new_y) {
            self.robot.move_robot(dx, dy);
        }
    }
}


struct App {
    gl: GlGraphics,
    world: World,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let world = &self.world;
        let robot = &world.robot;

        self.gl.draw(args.viewport(), |c, gl| {
            clear([1.0; 4], gl);

            // Draw grid
            for i in 0..GRID_SIZE {
                for j in 0..GRID_SIZE {
                    rectangle(
                        [0.8, 0.8, 0.8, 1.0], // Gray color for the grid
                        [
                            i as f64 * CELL_SIZE,
                            j as f64 * CELL_SIZE,
                            CELL_SIZE,
                            CELL_SIZE,
                        ],
                        c.transform,
                        gl,
                    );
                }
            }

            // Draw obstacles
            for &(x, y) in &world.obstacles {
                rectangle(
                    [1.0, 0.0, 0.0, 1.0], // Red color for obstacles
                    [
                        x as f64 * CELL_SIZE,
                        y as f64 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                    ],
                    c.transform,
                    gl,
                );
            }

            // Draw robot
            rectangle(
                [0.0, 0.0, 1.0, 1.0], // Blue color for the robot
                [
                    robot.x as f64 * CELL_SIZE,
                    robot.y as f64 * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                ],
                c.transform,
                gl,
            );
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {}

    fn press(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Up => self.world.move_robot(0, -1),
                Key::Down => self.world.move_robot(0, 1),
                Key::Left => self.world.move_robot(-1, 0),
                Key::Right => self.world.move_robot(1, 0),
                _ => {}
            }
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Robot Simulator", [400, 400])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        world: World::new(),
    };

    let mut events = Events::new(EventSettings::new()).ups(60);

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.press(&Button::Keyboard(key));
        }
    }
}

