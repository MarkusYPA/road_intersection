use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use rand::Rng;

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
    position: (u8, u8),
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

    fn print_state(&self) {
        println!("--------------------");
        for (i, street) in self.streets.iter().enumerate() {
            println!("Street {}", i + 1);
            for (direction, (light, vehicles)) in &street.lanes {
                let light_state = match light {
                    TrafficLight::Red => "Red",
                    TrafficLight::Green => "Green",
                };
                println!("  {:?}: {:<5} - Vehicles: {}", direction, light_state, vehicles.len());
            }
        }
        println!("Timer: {}", self.timer);
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
                        if self.grid[next_pos.0 as usize][next_pos.1 as usize] == 0 {
                            self.grid[vehicle.position.0 as usize][vehicle.position.1 as usize] = 0;
                            vehicle.position = next_pos;
                            self.grid[next_pos.0 as usize][next_pos.1 as usize] = 1;
                        }

                        if next_pos.0 > 9 || next_pos.1 > 9 {
                            vehicle.active = false;
                            self.grid[vehicle.position.0 as usize][vehicle.position.1 as usize] = 0;
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
            let vehicle = Vehicle { position, route, direction, active: true };
            self.streets[street_idx]
                .lanes
                .get_mut(&direction)
                .unwrap()
                .1
                .push(vehicle);
            self.grid[position.0 as usize][position.1 as usize] = 1;
        }
    }

    fn run(&mut self) {
        for _ in 0..100 {
            self.print_state();
            if rand::thread_rng().gen_bool(0.3) { // 30% chance to spawn a vehicle
                self.spawn_vehicle(None);
            }
            self.update();
            thread::sleep(Duration::from_millis(500));
        }
    }
}

fn get_next_position(pos: (u8, u8), dir: Direction, route: Route, light: &TrafficLight, grid: &[[u8; 10]; 10]) -> (u8, u8) {
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
            Route::Straight => (x, y.saturating_sub(1)),
            Route::Left => if y > 4 { (x, y.saturating_sub(1)) } else { (x.saturating_sub(1), y) },
            Route::Right => if y > 4 { (x, y.saturating_sub(1)) } else { (x.saturating_add(1), y) },
        },
        Direction::South => match route {
            Route::Straight => (x, y.saturating_add(1)),
            Route::Left => if y < 5 { (x, y.saturating_add(1)) } else { (x.saturating_add(1), y) },
            Route::Right => if y < 5 { (x, y.saturating_add(1)) } else { (x.saturating_sub(1), y) },
        },
        Direction::East => match route {
            Route::Straight => (x.saturating_add(1), y),
            Route::Left => if x < 5 { (x.saturating_add(1), y) } else { (x, y.saturating_sub(1)) },
            Route::Right => if x < 5 { (x.saturating_add(1), y) } else { (x, y.saturating_add(1)) },
        },
        Direction::West => match route {
            Route::Straight => (x.saturating_sub(1), y),
            Route::Left => if x > 4 { (x.saturating_sub(1), y) } else { (x, y.saturating_add(1)) },
            Route::Right => if x > 4 { (x.saturating_sub(1), y) } else { (x, y.saturating_sub(1)) },
        },
    };

    if next_pos.0 < 10 && next_pos.1 < 10 && grid[next_pos.0 as usize][next_pos.1 as usize] == 0 {
        next_pos
    } else {
        pos
    }
}

fn main() {
    let mut intersection = Intersection::new();
    intersection.run();
}