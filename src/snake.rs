use crate::constants::{GRID_COLS, GRID_ROWS, SIZE, SPEED};
use crate::utils::{wrap_x, wrap_y, Direction, Vector};
use sdl2::render::TextureQuery;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::WindowCanvas};

pub struct Snake {
    pub body: Vec<Vector>,
    direction: Direction,
    current_direction: Direction,
    pub path: Vec<Vector>,
    score: u32,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            body: vec![Vector { x: 32, y: 18 }],
            direction: Direction::RIGHT,
            current_direction: Direction::RIGHT,
            path: Vec::new(),
            score: 0,
        }
    }

    pub fn should_find_path(&self) -> bool {
        self.path.is_empty()
    }

    pub fn set_path(&mut self, path: Vec<Vector>) {
        self.path = path;
    }

    pub fn event_handler(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => {
                self.direction = Direction::UP;
            }
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => {
                self.direction = Direction::LEFT;
            }
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => {
                self.direction = Direction::DOWN;
            }
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => {
                self.direction = Direction::RIGHT;
            }
            _ => {}
        }
    }

    pub fn get_direction(&self, new_position: Vector) -> Direction {
        let mut direction = self.current_direction;
        if self.body[0].x == new_position.x {
            direction = if wrap_y!(self.body[0].y - 1) == new_position.y {
                Direction::UP
            } else {
                Direction::DOWN
            };
        } else if self.body[0].y == new_position.y {
            direction = if wrap_x!(self.body[0].x - 1) == new_position.x {
                Direction::LEFT
            } else {
                Direction::RIGHT
            };
        }
        direction
    }

    pub fn update(&mut self, astar: bool) {
        if astar {
            if let Some(pos) = self.path.pop() {
                self.direction = self.get_direction(pos);
            } else {
                self.direction = self.get_direction(Vector { x: 0, y: 0 });
            }
        }

        if self.direction != !self.current_direction {
            self.current_direction = self.direction;
        }

        self.update_body();
    }

    fn update_body(&mut self) {
        for i in (1..self.body.len()).rev() {
            self.body[i] = self.body[i - 1];
        }
        let sign = match self.current_direction {
            Direction::UP | Direction::LEFT => 1,
            Direction::DOWN | Direction::RIGHT => -1,
        };

        match self.current_direction {
            Direction::UP | Direction::DOWN => {
                self.body[0].y = wrap_y!(self.body[0].y - sign * SPEED)
            }
            Direction::RIGHT | Direction::LEFT => {
                self.body[0].x = wrap_x!(self.body[0].x - sign * SPEED)
            }
        }
    }

    pub fn check_collision_with_head(&self, position: Vector) -> bool {
        self.body[0] == position
    }

    pub fn check_collision_with_snake(&self, position: Vector) -> bool {
        self.body.iter().any(|&pos| pos == position)
    }

    pub fn check_self_collision(&self) -> bool {
        self.body
            .iter()
            .skip(1)
            .any(|&pos| self.check_collision_with_head(pos))
    }

    pub fn grow(&mut self) {
        // New body part will be added to the end of the snake at the next update
        // Add at arbitary position for now
        self.body.push(Vector { x: -1, y: -1 });
        self.score += 1;
    }

    pub fn draw_score(&mut self, canvas: &mut WindowCanvas) {
        // Draw the score at the top left corner
        let score = format!("Score: {}", self.score);
        let font_context = sdl2::ttf::init().unwrap();
        let font = font_context
            .load_font("assets/Poppins-Bold.ttf", SIZE as u16)
            .unwrap();
        let surface = font.render(&score).blended(Color::RGB(0, 0, 0)).unwrap();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new(0, 0, width, height);

        canvas.copy(&texture, None, Some(target)).unwrap();
    }

    pub fn draw(&mut self, canvas: &mut WindowCanvas) {
        // Draw the head
        canvas.set_draw_color(Color::RGB(0, 150, 0));
        let mut rect = Rect::new(
            self.body[0].x * SIZE as i32,
            self.body[0].y * SIZE as i32,
            SIZE,
            SIZE,
        );
        canvas.fill_rect(rect).expect("Failed to draw snake head.");

        // Draw rest of the body
        canvas.set_draw_color(Color::GREEN);
        for &pos in self.body.iter().skip(1) {
            rect = Rect::new(pos.x * SIZE as i32, pos.y * SIZE as i32, SIZE, SIZE);
            canvas.fill_rect(rect).expect("Failed to draw snake body.");
        }

        self.draw_score(canvas);
    }
}
