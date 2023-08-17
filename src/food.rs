use crate::constants::{GRID_COLS, GRID_ROWS, SIZE};
use crate::utils::Vector;
use rand::{rngs::ThreadRng, Rng};
use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas};

pub struct Food {
    pub position: Vector,
    rng: ThreadRng,
}

impl Food {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            position: Vector {
                x: rng.gen_range(0..GRID_COLS),
                y: rng.gen_range(0..GRID_ROWS),
            },
            rng: rng,
        }
    }

    pub fn spawn(&mut self) {
        self.position.x = self.rng.gen_range(0..GRID_COLS);
        self.position.y = self.rng.gen_range(0..GRID_ROWS);
    }

    pub fn draw(&mut self, canvas: &mut WindowCanvas) {
        let rect = Rect::new(self.position.x * SIZE as i32, self.position.y * SIZE as i32, SIZE, SIZE);
        canvas.set_draw_color(Color::RED);
        canvas.fill_rect(rect).expect("Failed to draw food.");
    }
}
