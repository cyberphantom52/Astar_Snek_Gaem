use crate::constants::{GRID_COLS, GRID_ROWS, HEIGHT, WIDTH};
use crate::{
    astar::find_path,
    food::Food,
    snake::Snake,
    utils::{Block, Map, Vector},
};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, render::WindowCanvas, EventPump};

#[derive(PartialEq)]
pub enum GAMESTATE {
    READY,
    RUNNING,
    GAMEOVER,
}

pub struct Game {
    canvas: WindowCanvas,
    event_pump: EventPump,
    state: GAMESTATE,
    snake: Snake,
    food: Food,
    pub astar: bool,
}

impl Game {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Snek Gaem", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .expect("Could not initialize video subsystem");

        let canvas = window.into_canvas().build().expect("could not make canvas");

        let event_pump = sdl_context
            .event_pump()
            .expect("Failed to initialize event pump");

        Self {
            canvas: canvas,
            event_pump: event_pump,
            state: GAMESTATE::READY,
            snake: Snake::new(),
            food: Food::new(),
            astar: false,
        }
    }

    fn event_handler(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.state = GAMESTATE::GAMEOVER,
                Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
                } => {
                    self.snake.set_path(Vec::new());
                    self.astar = !self.astar;
                }
                _ => {}
            }

            if !self.astar {
                self.snake.event_handler(&event);
            }
        }
    }

    fn make_map(&self) -> Map {
        let mut map = vec![vec![Block::new(0, 0); GRID_COLS as usize]; GRID_ROWS as usize];
        for (y, row) in map.iter_mut().enumerate() {
            for (x, block) in row.iter_mut().enumerate() {
                block.pos = Vector {
                    x: x as i32,
                    y: y as i32,
                };
            }
        }

        for body_part in &self.snake.body {
            let x = body_part.x as usize;
            let y = body_part.y as usize;
            if let Some(block) = map.get_mut(y).and_then(|row| row.get_mut(x)) {
                block.is_wall = true;
            }
        }

        map
    }

    fn update(&mut self) {
        if self.astar && self.snake.should_find_path() {
            if let Some(path) = find_path(&mut self.make_map(), self.snake.body[0], self.food.position) {
                self.snake.set_path(path);
            } else {
                // No path was found; handle this case
                println!("GAME OVER! score: {}", self.snake.body.len() - 1);
                println!("No path found.");
                self.state = GAMESTATE::GAMEOVER;
            }
        }

        self.snake.update(self.astar);

        if self.snake.check_collision_with_head(self.food.position) {
            while self.snake.check_collision_with_snake(self.food.position) {
                self.food.spawn();
            }
            self.snake.grow();
        }

        if self.snake.check_self_collision() {
            self.state = GAMESTATE::GAMEOVER;
        }
    }

    fn draw(&mut self) {
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();
        self.snake.draw(&mut self.canvas);
        self.food.draw(&mut self.canvas);
        self.canvas.present();
    }

    pub fn start(&mut self) {
        self.state = GAMESTATE::RUNNING;
        while self.state == GAMESTATE::RUNNING {
            self.event_handler();
            self.update();
            self.draw();
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}
