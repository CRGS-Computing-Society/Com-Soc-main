use ggez::*;
use rand::Rng;

const GRID_SIZE: (usize, usize) = (20, 20);
const CELL_SIZE: (usize, usize) = (16, 16);

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * CELL_SIZE.1 as f32,
);

const UPDATE_RATE: f32 = 16.0;
const MILLIS_PER_UPDATE: f32 = (1.0 / UPDATE_RATE * 1000.0);

const COLORS: [(u8, u8, u8); 2] = [
    (0, 0, 0),
    (255, 0, 0)
];

const USE_PATHFINDING: bool = true;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridPosition {
    x: usize,
    y: usize,
}


impl GridPosition {
    pub fn new(x: usize, y: usize) -> GridPosition {
        GridPosition {x, y}
    }

    pub fn random(min_x: usize, max_x: usize, min_y:usize, max_y: usize) -> GridPosition {
        let mut rng = rand::thread_rng();
        (
            rng.gen_range::<usize, usize, usize>(min_x, max_x),
            rng.gen_range::<usize, usize, usize>(min_y, max_y)
        ).into()
    }
}

impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> graphics::Rect {
        graphics::Rect::new_i32(
            pos.x as i32 * CELL_SIZE.0 as i32,
            pos.y as i32 * CELL_SIZE.0 as i32,
            CELL_SIZE.0 as i32,
            CELL_SIZE.1 as i32,
        )
    }
}

impl From<(usize, usize)> for GridPosition {
    fn from(pos: (usize, usize)) -> GridPosition {
        GridPosition {x: pos.0, y: pos.1}
    }
}

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("traffic", "Pratyaksh Sharma")
        .window_setup(conf::WindowSetup::default().title("Traffic Simulator"))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1)) 
        .build()
        .expect("Could not create ggez context.");

    let mut state = MainState::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut state) {
        Ok(_) => println!("Exited cleanly"),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct MainState {
    dt: std::time::Duration,
    ticks: usize,
    display_grid: Vec<Vec<i32>>,
    heatmap: Vec<Vec<usize>>,
    cars: Vec<Car>,
    stuck_count: i32
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> MainState {
        let cars: Vec<Car> = Vec::new();
        let heatmap: Vec<Vec<usize>> = vec![vec![0; GRID_SIZE.0]; GRID_SIZE.1];

        MainState {
            dt: std::time::Duration::new(0, 0),
            ticks: 0,
            display_grid: vec![vec![0; GRID_SIZE.0]; GRID_SIZE.1],
            heatmap,
            cars,
            stuck_count: 0
        }
    }

    pub fn calculate_new_routes(&mut self) {
        for car in &mut self.cars {
            let mut dp: Vec<Vec<usize>> = vec![vec![0; GRID_SIZE.0]; GRID_SIZE.1];
            let mut directions: Vec<Vec<GridPosition>> = vec![vec![GridPosition::new(0, 0); GRID_SIZE.0]; GRID_SIZE.1];
            let mut order: Vec<Vec<GridPosition>> = vec![Vec::new(); GRID_SIZE.0 + GRID_SIZE.1 + 1];
            
            for r in 0..GRID_SIZE.1 {
                for c in 0..GRID_SIZE.0 {
                    dp[r][c] = GRID_SIZE.0 * GRID_SIZE.1 + 1;
                    order[(r as i32 - car.current_pos.y as i32).abs() as usize + (c as i32 - car.current_pos.x as i32).abs() as usize].push(GridPosition::new(c, r))
                }
            }

            dp[car.current_pos.y][car.current_pos.x] = 0;
            
            for dist in order {
                for pos in dist {
                    let mut dir = GridPosition::new(0, 0);
                    let mut min = dp[pos.y][pos.x];

                    if USE_PATHFINDING {
                        if pos.y > 0 && dp[pos.y - 1][pos.x] + self.heatmap[pos.y - 1][pos.x] < min {
                            min = dp[pos.y - 1][pos.x] + self.heatmap[pos.y - 1][pos.x];
                            dir = GridPosition::new(pos.x, pos.y - 1);
                        }

                        if pos.y < GRID_SIZE.1 - 1 && dp[pos.y + 1][pos.x] + self.heatmap[pos.y + 1][pos.x] < min {
                            min = dp[pos.y + 1][pos.x] + self.heatmap[pos.y + 1][pos.x];
                            dir = GridPosition::new(pos.x, pos.y + 1);
                        }

                        if pos.x > 0 && dp[pos.y][pos.x - 1] + self.heatmap[pos.y][pos.x - 1] < min {
                            min = dp[pos.y][pos.x - 1] + self.heatmap[pos.y][pos.x - 1];
                            dir = GridPosition::new(pos.x - 1, pos.y);
                        }

                        if pos.x < GRID_SIZE.0 - 1 && dp[pos.y][pos.x + 1] + self.heatmap[pos.y][pos.x + 1] < min {
                            min = dp[pos.y][pos.x + 1] + self.heatmap[pos.y][pos.x + 1];
                            dir = GridPosition::new(pos.x + 1, pos.y);
                        }    
                    } else {
                        // Turn off pathfinding - do not consider congestion values
                        if pos.y > 0 && dp[pos.y - 1][pos.x] < min {
                            min = dp[pos.y - 1][pos.x];
                            dir = GridPosition::new(pos.x, pos.y - 1);
                        }

                        if pos.y < GRID_SIZE.1 - 1 && dp[pos.y + 1][pos.x] < min {
                            min = dp[pos.y + 1][pos.x];
                            dir = GridPosition::new(pos.x, pos.y + 1);
                        }

                        if pos.x > 0 && dp[pos.y][pos.x - 1] < min {
                            min = dp[pos.y][pos.x - 1];
                            dir = GridPosition::new(pos.x - 1, pos.y);
                        }

                        if pos.x < GRID_SIZE.0 - 1 && dp[pos.y][pos.x + 1] < min {
                            min = dp[pos.y][pos.x + 1];
                            dir = GridPosition::new(pos.x + 1, pos.y);
                        }
                    }
                    

                    directions[pos.y][pos.x] = dir;
                    dp[pos.y][pos.x] = min + 1;
                }
            }

            let mut current = car.destination_pos;
            let mut route: Vec<GridPosition> = Vec::new();
            while current != car.current_pos {
                route.push(current);
                current = directions[current.y][current.x];
            }

            while route.len() > 0 {
                car.route.push(route.pop().unwrap());
            }
        }
    }
}

#[derive(Clone)]
struct Car {
    current_pos: GridPosition,
    destination_pos: GridPosition,
    route: Vec<GridPosition>
}

impl Car {
    fn new() -> Car {
        Car {
            current_pos: GridPosition::random(0, GRID_SIZE.0, 0, GRID_SIZE.1),
            destination_pos: GridPosition::random(0, GRID_SIZE.0, 0, GRID_SIZE.1), // replace with GridPosition::new(10, 10) for commuter mode (all cars going to centre)
            route: Vec::new(),
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt += timer::delta(ctx);
        if self.dt.as_millis() as f32 > MILLIS_PER_UPDATE {
            self.dt = std::time::Duration::new(0, 0);

            if self.ticks % 2 == 0 {
                self.cars.push(Car::new());
            }

            if self.ticks > 0 && self.ticks % 1000 == 0 {
                println!("Average stuck cars: {}", self.stuck_count as f32 / self.ticks as f32);
            }
            
            self.calculate_new_routes();
            
            self.ticks += 1;
            let mut new_cars: Vec<Car> = Vec::new();
            let mut rng = rand::thread_rng();

            for i in 0..self.cars.len() {
                if self.cars[i].route.len() > 0 {
                    let next_pos = self.cars[i].route[0];
                    if self.heatmap[next_pos.y][next_pos.x] <= rng.gen_range::<usize, usize, usize>(0, 20) {
                        // Move to next position and remove one cell from path
                        self.display_grid[self.cars[i].current_pos.y][self.cars[i].current_pos.x] = 0;
                        self.cars[i].current_pos = next_pos;
                        self.display_grid[next_pos.y][next_pos.x] = 1;
                        self.cars[i].route.remove(0);

                        // Add to heatmap when a car moves to its next position
                        self.heatmap[next_pos.y][next_pos.x] += 5;
                        if next_pos.y > 0 { self.heatmap[next_pos.y - 1][next_pos.x] += 5; }
                        if next_pos.y < GRID_SIZE.1 - 1 { self.heatmap[next_pos.y + 1][next_pos.x] += 5; }
                        if next_pos.x > 0 { self.heatmap[next_pos.y][next_pos.x - 1] += 5; }
                        if next_pos.x < GRID_SIZE.0 - 1 { self.heatmap[next_pos.y][next_pos.x + 1] += 5; }

                        if next_pos == self.cars[i].destination_pos {
                            self.display_grid[next_pos.y][next_pos.x] = 0;
                            // Finished cars don't get pushed onto next tick
                            continue;
                        }
                    } else {
                        // Can't move - car got stuck
                        self.stuck_count += 1;
                    }

                    new_cars.push(self.cars[i].clone());
                }
            }

            for r in 0..GRID_SIZE.1 {
                for c in 0..GRID_SIZE.0 {
                    if self.heatmap[r][c] > 0 {
                        self.heatmap[r][c] -= 1;
                    }
                }
            }

            self.cars = new_cars;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        for r in 0..GRID_SIZE.1 {
            for c in 0..GRID_SIZE.0 {
                if self.display_grid[r][c] != 0 {
                    let rect = graphics::Mesh::new_rectangle(
                        ctx, 
                        graphics::DrawMode::fill(), 
                        GridPosition::new(c, r).into(), 
                        COLORS[self.display_grid[r][c] as usize - 1].into()
                    )?;
                    graphics::draw(ctx, &rect, (nalgebra::Point2::new(0.0, 0.0),))?;
                } else {
                    let rect = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        GridPosition::new(c, r).into(),
                        (255 as u8, 0 as u8, 0 as u8, std::cmp::min(255, self.heatmap[r][c] * 10) as u8).into()
                    )?;
                    graphics::draw(ctx, &rect, (nalgebra::Point2::new(0.0, 0.0),))?;
                }
            }
        }
        graphics::present(ctx)
    }
}
