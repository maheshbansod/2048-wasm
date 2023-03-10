mod grid;
mod utils;

use grid::{Grid, GridCoord, MoveTransition};
use serde::{Deserialize, Serialize};
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState {
    Playing,
    Over,
}

#[derive(Serialize, Deserialize)]
pub struct GameUpdate {
    moves: Vec<MoveTransition>,
    new_piece: GridCoord,
    score: u32,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        set_panic_hook();
        Self::new_game()
    }

    pub fn reset(&mut self) {
        let game = Self::new_game();
        self.grid = game.grid;
        self.score = game.score;
        self.state = game.state;
    }

    fn new_game() -> Self {
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

    pub fn play_swipe(&mut self, direction: Direction) -> Result<JsValue, String> {
        if self.state == GameState::Over {
            return Err("The game is over".into());
        }
        if let Ok((score, moves)) = self.grid.move_cells(direction) {
            self.score += score;
            // add a random piece
            let new_piece = self.grid.add_at_random_position(Game::get_small_piece())?; // will never fail - since movement means at least one place was free
            if self.grid.is_game_over() {
                self.state = GameState::Over;
            }
            let update = GameUpdate {
                moves,
                score,
                new_piece,
            };
            Ok(serde_wasm_bindgen::to_value(&update).unwrap())
        } else {
            Err("Illegal move".into())
        }
    }

    pub fn cells(&self) -> *const Cell {
        self.grid.cells()
    }

    fn get_small_piece() -> u32 {
        let pieces = [2, 4];
        let idx = (js_sys::Math::random() * 2f64).floor() as usize;
        pieces[idx]
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn state(&self) -> GameState {
        self.state
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
