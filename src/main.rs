mod astar;
mod constants;
mod food;
mod game;
mod snake;
mod utils;

use crate::game::Game;

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_default();
    let mut game = Game::new();
    if arg == "astar" {
        game.astar = true;
    }
    game.start();
}
