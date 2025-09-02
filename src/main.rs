use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy)]
enum Route {
    Straight,
    Left,
    Right,
}

#[derive(Debug)]
enum TrafficLight {
    Red,
    Green,
}

#[derive(Debug, Clone, Copy)]
struct Vehicle {
    position: (i32, i32),
    route: Route,
    direction: Direction,
    active: bool,
}

struct Street {
    lanes: HashMap<Direction, (TrafficLight, Vec<Vehicle>)>
}

impl Street {
    fn new(directions: Vec<Direction>) -> Self {
        let mut lanes = HashMap::new();
        for dir in directions {
            lanes.insert(dir, (TrafficLight::Red, Vec::new()));
        }
        Self { lanes }
    }
}

struct Intersection {
    streets: Vec<Street>,
    timer: u64,
    grid: [[u8; 10]; 10],
}

impl Intersection {
    fn new() -> Self {
        let mut streets = Vec::new();
        streets.push(Street::new(vec![Direction::North, Direction::South]));
        streets.push(Street::new(vec![Direction::East, Direction::West]));
        streets[0].lanes.get_mut(&Direction::North).unwrap().0 = TrafficLight::Green;
        streets[0].lanes.get_mut(&Direction::South).unwrap().0 = TrafficLight::Green;
        Self { streets, timer: 0, grid: [[0; 10]; 10] }
    }

    fn update(&mut self) {
        self.timer += 1;
        self.grid = [[0; 10]; 10];

        // Move vehicles
        for street in &mut self.streets {
            for (_direction, (light, vehicles)) in street.lanes.iter_mut() {
                vehicles.retain(|v| v.active);

                for vehicle in vehicles.iter_mut() {
                    if vehicle.active {
                        let next_pos = get_next_position(vehicle.position, vehicle.direction, vehicle.route, light, &self.grid);

                        if next_pos.0 < 0 || next_pos.0 > 9 || next_pos.1 < 0 || next_pos.1 > 9 {
                            println!("Removing vehicle at {:?}", vehicle.position);
                            vehicle.active = false;
                            self.grid[vehicle.position.0 as usize][vehicle.position.1 as usize] = 0;
                            continue;
                        }

                        // Only move if grid place is unoccupied
                        if self.grid[next_pos.0 as usize][next_pos.1 as usize] == 0 {
                            self.grid[vehicle.position.0 as usize][vehicle.position.1 as usize] = 0;
                            vehicle.position = next_pos;
                            self.grid[next_pos.0 as usize][next_pos.1 as usize] = 1;
                        }


                    }
                }
            }
        }

        if self.timer % 10 == 0 {
            for street in &mut self.streets {
                for (light, _) in street.lanes.values_mut() {
                    *light = match light {
                        TrafficLight::Red => TrafficLight::Green,
                        TrafficLight::Green => TrafficLight::Red,
                    };
                }
            }
        }
    }

    fn spawn_vehicle(&mut self, direction: Option<Direction>) {
        let mut rng = rand::thread_rng();
        let direction = direction.unwrap_or_else(|| match rng.gen_range(0..4) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            3 => Direction::West,
            _ => unreachable!(),
        });

        let route = match rng.gen_range(0..3) {
            0 => Route::Straight,
            1 => Route::Left,
            2 => Route::Right,
            _ => unreachable!(),
        };

        let position = match direction {
            Direction::North => (4, 9),
            Direction::South => (5, 0),
            Direction::East => (0, 4),
            Direction::West => (9, 5),
        };

        let street_idx = if direction == Direction::North || direction == Direction::South { 0 } else { 1 };

        if self.grid[position.0 as usize][position.1 as usize] == 0 {
            let vehicle = Vehicle { position: (position.0 as i32, position.1 as i32), route, direction, active: true };
            self.streets[street_idx]
                .lanes
                .get_mut(&direction)
                .unwrap()
                .1
                .push(vehicle);
            self.grid[position.0 as usize][position.1 as usize] = 1;
        }
    }

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw roads
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.fill_rect(Rect::new(350, 0, 100, 600))?;
        canvas.fill_rect(Rect::new(0, 250, 800, 100))?;

        // Draw traffic lights
        for (i, street) in self.streets.iter().enumerate() {
            for (direction, (light, _)) in &street.lanes {
                let color = match light {
                    TrafficLight::Red => Color::RGB(255, 0, 0),
                    TrafficLight::Green => Color::RGB(0, 255, 0),
                };
                canvas.set_draw_color(color);
                let rect = match (i, direction) {
                    (0, Direction::North) => Rect::new(400, 240, 10, 10),
                    (0, Direction::South) => Rect::new(440, 350, 10, 10),
                    (1, Direction::East) => Rect::new(340, 300, 10, 10),
                    (1, Direction::West) => Rect::new(450, 290, 10, 10),
                    _ => Rect::new(0, 0, 0, 0),
                };
                canvas.fill_rect(rect)?;
            }
        }

        // Draw vehicles
        for street in &self.streets {
            for (_, (_, vehicles)) in &street.lanes {
                for vehicle in vehicles {
                    let color = match vehicle.route {
                        Route::Straight => Color::RGB(255, 255, 0),
                        Route::Left => Color::RGB(0, 255, 255),
                        Route::Right => Color::RGB(255, 0, 255),
                    };
                    canvas.set_draw_color(color);
                    let rect = Rect::new(vehicle.position.0 * 80, vehicle.position.1 * 60, 20, 20);
                    canvas.fill_rect(rect)?;
                }
            }
        }

        canvas.present();
        Ok(())
    }
}

fn get_next_position(pos: (i32, i32), dir: Direction, route: Route, light: &TrafficLight, _grid: &[[u8; 10]; 10]) -> (i32, i32) {
    let (x, y) = pos;

    match light {
        TrafficLight::Red => {
            if (dir == Direction::North && y == 6) || (dir == Direction::South && y == 3) || (dir == Direction::East && x == 3) || (dir == Direction::West && x == 6) {
                return pos;
            }
        }
        TrafficLight::Green => {}
    }

    let next_pos = match dir {
        Direction::North => match route {
            Route::Straight => (x, y - 1),
            Route::Left => if y > 4 { (x, y - 1) } else { (x - 1, y) },
            Route::Right => if y > 4 { (x, y - 1) } else { (x + 1, y) },
        },
        Direction::South => match route {
            Route::Straight => (x, y + 1),
            Route::Left => if y < 5 { (x, y + 1) } else { (x + 1, y) },
            Route::Right => if y < 5 { (x, y + 1) } else { (x - 1, y) },
        },
        Direction::East => match route {
            Route::Straight => (x + 1, y),
            Route::Left => if y < 5 { (x + 1, y) } else { (x, y - 1) },
            Route::Right => if y < 5 { (x + 1, y) } else { (x, y + 1) },
        },
        Direction::West => match route {
            Route::Straight => (x - 1, y),
            Route::Left => if x > 4 { (x - 1, y) } else { (x, y + 1) },
            Route::Right => if x > 4 { (x - 1, y) } else { (x, y - 1) },
        },
    };

    // position validation not necessary 
    /* if next_pos.0 >= 0 && next_pos.0 < 10 && next_pos.1 >= 0 && next_pos.1 < 10 && grid[next_pos.0 as usize][next_pos.1 as usize] == 0 {
        next_pos
    } else {
        pos
    } */

    next_pos
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Road Intersection", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut intersection = Intersection::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    intersection.spawn_vehicle(Some(Direction::South));
                },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    intersection.spawn_vehicle(Some(Direction::North));
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    intersection.spawn_vehicle(Some(Direction::West));
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    intersection.spawn_vehicle(Some(Direction::East));
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    intersection.spawn_vehicle(None);
                },
                _ => {}
            }
        }

        intersection.update();
        intersection.render(&mut canvas)?;

        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}