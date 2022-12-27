mod grid;
mod utils;

use grid::Grid;
use rand::seq::SliceRandom;
use wasm_bindgen::prelude::*;

use crate::{grid::Direction, utils::set_panic_hook};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Game {
    score: u32,
    grid: Grid,
    state: GameState,
}

#[wasm_bindgen]
#[derive(PartialEq, Eq)]
pub enum GameState {
    Playing,
    Over,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        set_panic_hook();
        let mut game = Self {
            state: GameState::Playing,
            score: 0,
            grid: Grid::new(4),
        };
        game.grid
            .add_at_random_position(Game::get_small_piece())
            .unwrap();
        game
    }

    pub fn play_swipe(&mut self, direction: Direction) -> Result<(), String> {
        if self.state == GameState::Over {
            return Err("The game is over".into());
        }
        let score = self.grid.move_cells(direction);
        self.score += score;
        // add a random piece
        if self
            .grid
            .add_at_random_position(Game::get_small_piece())
            .is_err()
        {
            // game over since we can't add a piece
            self.state = GameState::Over;
        }
        Ok(())
    }

    fn get_small_piece() -> u32 {
        let pieces = [2, 4];
        *pieces.choose(&mut rand::thread_rng()).unwrap()
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
