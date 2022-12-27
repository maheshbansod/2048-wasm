mod grid;
mod utils;

use grid::Grid;
use wasm_bindgen::prelude::*;

use crate::{
    grid::{Cell, Direction},
    utils::set_panic_hook,
};

extern crate js_sys;
extern crate web_sys;

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
        let score = if let Ok(score) = self.grid.move_cells(direction) {
            score
        } else {
            return Err("Illegal move".into());
        };
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

    pub fn cells(&self) -> *const Cell {
        self.grid.cells()
    }

    fn get_small_piece() -> u32 {
        let pieces = [2, 4];
        let idx = (js_sys::Math::random() * 2f64).floor() as usize;
        pieces[idx]
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
