use sfml::{
    graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable, View},
    system::{Clock, Time, Vector2, Vector2f, Vector2i},
    window::{ContextSettings, Event, Key, Style},
};

struct BoolGrid2D {
    array: Vec<bool>,
    width: usize,
    height: usize,
}

impl BoolGrid2D {
    fn new(width: usize, height: usize) -> BoolGrid2D {
        BoolGrid2D {
            array: vec![false; width * height],
            width,
            height,
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    fn get(&self, x: usize, y: usize) -> bool {
        let index = self.get_index(x, y);
        self.array[index]
    }

    fn set(&mut self, x: usize, y: usize, val: bool) {
        let index = self.get_index(x, y);
        self.array[index] = val;
    }
}

struct Game {
    grid: BoolGrid2D,
    simulation_grid: BoolGrid2D,
    paused: bool,
    cell_size: Vector2f,
    cell_color: Color,
}

impl Game {
    fn new(width: usize, height: usize) -> Game {
        Game {
            grid: BoolGrid2D::new(width, height),
            simulation_grid: BoolGrid2D::new(width, height),
            paused: false,
            cell_size: Vector2f::new(10.0, 10.0),
            cell_color: Color::rgb(255, 255, 255),
        }
    }

    fn get_cell_below_position(&self, position: Vector2f) -> Vector2<usize> {
        Vector2::new(
            (position.x / self.cell_size.x).floor() as usize,
            (position.y / self.cell_size.y).floor() as usize,
        )
    }

    fn toggle_cell(&mut self, position: Vector2<usize>) {
        self.grid.set(
            position.x,
            position.y,
            !self.grid.get(position.x, position.y),
        );
    }

    fn get_neighbors_count(&self, x: usize, y: usize) -> i32 {
        let mut count = 0;

        let grid_width = self.grid.width;
        let grid_height = self.grid.height;

        // Check top and bottom
        if y != 0 && self.grid.get(x, y - 1) {
            count += 1
        }
        if y != grid_height - 1 && self.grid.get(x, y + 1) {
            count += 1
        }

        // Check right and left
        if x != grid_width - 1 && self.grid.get(x + 1, y) {
            count += 1
        }
        if x != 0 && self.grid.get(x - 1, y) {
            count += 1
        }

        // Check top left and top right
        if x != 0 && y != 0 && self.grid.get(x - 1, y - 1) {
            count += 1
        }
        if x != grid_width - 1 && y != 0 && self.grid.get(x + 1, y - 1) {
            count += 1
        }

        // Check bottom right and bottom left
        if x != grid_width - 1 && y != grid_height - 1 && self.grid.get(x + 1, y + 1) {
            count += 1
        }
        if x != 0 && y != grid_height - 1 && self.grid.get(x - 1, y + 1) {
            count += 1
        }

        count
    }

    fn apply_simulation_grid(&mut self) {
        self.grid.array = self.simulation_grid.array.clone();
    }

    fn process_event(&mut self, event: &Event, window: &RenderWindow) {
        match event {
            Event::MouseButtonPressed { x, y, .. } => {
                let mouse_pos = window.map_pixel_to_coords(Vector2i::new(*x, *y), window.view());
                let cell_pos = self.get_cell_below_position(mouse_pos);
                self.toggle_cell(cell_pos);
            }
            Event::KeyPressed { code, .. } => {
                if Key::SPACE == *code {
                    self.paused = !self.paused;
                }
            }
            _ => (),
        }
    }

    fn update(&mut self) {
        if self.paused {
            return;
        }

        let grid = &self.grid;

        for y in 0..grid.height {
            for x in 0..grid.width {
                let is_alive = grid.get(x, y);
                let neighbors_count = self.get_neighbors_count(x, y);

                let mut is_going_to_live = false;

                if is_alive {
                    if neighbors_count < 2 {
                        is_going_to_live = false;
                    } else if neighbors_count == 2 || neighbors_count == 3 {
                        is_going_to_live = true;
                    } else if neighbors_count > 3 {
                        is_going_to_live = false;
                    }
                } else {
                    if neighbors_count == 3 {
                        is_going_to_live = true;
                    }
                }

                self.simulation_grid.set(x, y, is_going_to_live);
            }
        }

        self.apply_simulation_grid();
    }

    fn draw(&self, target: &mut impl RenderTarget) {
        let grid = &self.grid;

        let mut cell_shape = RectangleShape::new();

        cell_shape.set_size(self.cell_size);
        cell_shape.set_fill_color(self.cell_color);

        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.get(x, y) {
                    cell_shape
                        .set_position((x as f32 * self.cell_size.x, y as f32 * self.cell_size.y));
                    target.draw(&cell_shape);
                }
            }
        }
    }
}

fn main() {
    let mut game = Game::new(40, 40);

    let mut window = RenderWindow::new(
        (400, 400),
        "Conway's Game of Life",
        Style::DEFAULT,
        &ContextSettings::default(),
    );

    window.set_framerate_limit(30);
    window.set_view(&View::new(
        Vector2f::new(200.0, 200.0),
        Vector2f::new(400.0, 400.0),
    ));

    let mut elapsed = Time::ZERO;
    let mut clock = Clock::start();

    let tick_duration = Time::seconds(0.2);
    let mut elapsed_since_last_tick = Time::ZERO;

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            game.process_event(&event, &window);

            match event {
                Event::Closed => window.close(),
                _ => (),
            }
        }

        if elapsed_since_last_tick > tick_duration {
            elapsed_since_last_tick -= tick_duration;

            game.update();
        }

        window.clear(Color::rgb(0, 0, 0));
        game.draw(&mut window);
        window.display();

        elapsed = clock.restart();
        elapsed_since_last_tick += elapsed;
    }
}
