mod utils;

use std::fmt::Display;

use rand::seq::SliceRandom;
use wasm_bindgen::prelude::*;

use crate::utils::set_panic_hook;

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

struct Grid {
    size: usize,
    grid: Vec<Cell>,
}

#[derive(Debug, Clone, Copy)]
struct GridCoord {
    x: usize,
    y: usize,
}

#[wasm_bindgen]
#[derive(PartialEq, Eq)]
pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    value: Option<u32>,
}

impl Grid {
    fn new(size: usize) -> Self {
        let grid = (0..size * size)
            .into_iter()
            .map(|_i| Cell { value: None })
            .collect::<Vec<Cell>>();
        Self { size, grid }
    }

    fn empty_cells(&self) -> Vec<GridCoord> {
        (0..self.size as usize)
            .flat_map(|i| {
                (0..self.size as usize)
                    .map(move |j| (i, j))
                    .filter(|&(i, j)| {
                        self.grid[self.get_index_from_coord((i, j).into())]
                            .value
                            .is_none()
                    })
            })
            .map(|x| x.into())
            .collect()
    }

    pub fn add_at_random_position(&mut self, value: u32) -> Result<(), String> {
        let empty_cells = self.empty_cells();
        let cell = empty_cells
            .choose(&mut rand::thread_rng())
            .ok_or_else(|| "No empty cell found apparently".to_string())?;
        let idx = self.get_index_from_coord(*cell);
        self.grid[idx].value = Some(value);
        Ok(())
    }

    // returns the score of the move
    pub fn move_cells(&mut self, direction: Direction) -> u32 {
        let mut score = 0;
        let is_vertical = direction == Direction::UP || direction == Direction::DOWN;
        let is_down_or_right = direction == Direction::RIGHT || direction == Direction::DOWN;
        for line in 0..self.size {
            let mut main_iter: Box<dyn Iterator<Item = usize>> = if is_down_or_right {
                Box::new((1..self.size).rev())
            } else {
                Box::new(0..self.size)
            };
            for i in &mut *main_iter {
                let last_coords = if is_vertical { (line, i) } else { (i, line) };
                let last_idx = self.get_index_from_coord(last_coords.into());
                let sub_iter: Box<dyn Iterator<Item = usize>> = if is_down_or_right {
                    Box::new((0..i).rev())
                } else {
                    Box::new((i + 1)..self.size)
                };
                for j in sub_iter {
                    let last_cell = self.grid[last_idx];
                    let coords = if is_vertical { (line, j) } else { (j, line) };
                    let idx = self.get_index_from_coord(coords.into());
                    let cell = self.grid[idx];
                    if let Some(val) = cell.value {
                        if let Some(last_val) = last_cell.value {
                            if last_val == val {
                                // add and store in last, and clear current
                                let new_val = 2 * val;
                                score += new_val;
                                self.grid[last_idx] = Cell {
                                    value: Some(new_val),
                                };
                                self.grid[idx] = Cell { value: None };
                            }
                            break;
                        } else {
                            // last cell is empty
                            // set last cell and clear current
                            self.grid[last_idx] = Cell { value: Some(val) };
                            self.grid[idx] = Cell { value: None };
                        }
                    }
                }
            }
        }
        score
    }

    fn get_index_from_coord(&self, coord: GridCoord) -> usize {
        coord.y * self.size + coord.x
    }
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
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

#[cfg(test)]
mod tests {
    use crate::{Cell, Direction, Grid};

    // helper functions
    impl Grid {
        fn new_grid(size: usize, grid: Vec<Cell>) -> Self {
            Self { size, grid }
        }

        fn get_grid(&self) -> Vec<u32> {
            self.grid
                .iter()
                .map(|x| if let Some(val) = x.value { val } else { 0 })
                .collect()
        }
    }

    #[test]
    fn swipe_right() {
        let mut grid = Grid::new_grid(
            4,
            vec![0, 2, 0, 0, 4, 0, 0, 0, 0, 2, 2, 0, 3, 4, 2, 2]
                .iter()
                .map(|&x| {
                    if x == 0 {
                        Cell { value: None }
                    } else {
                        Cell { value: Some(x) }
                    }
                })
                .collect(),
        );
        grid.move_cells(Direction::RIGHT);

        assert_eq!(
            vec![0, 0, 0, 2, 0, 0, 0, 4, 0, 0, 0, 4, 0, 3, 4, 4,],
            grid.get_grid()
        );
    }

    #[test]
    fn swipe_left() {
        let mut grid = Grid::new_grid(
            4,
            vec![0, 2, 0, 0, 4, 0, 0, 0, 0, 2, 2, 0, 3, 4, 2, 2]
                .iter()
                .map(|&x| {
                    if x == 0 {
                        Cell { value: None }
                    } else {
                        Cell { value: Some(x) }
                    }
                })
                .collect(),
        );
        grid.move_cells(Direction::LEFT);

        assert_eq!(
            vec![2, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 3, 4, 4, 0],
            grid.get_grid()
        );
    }

    #[test]
    fn swipe_up() {
        let mut grid = Grid::new_grid(
            4,
            vec![0, 2, 0, 0, 4, 0, 0, 0, 0, 2, 2, 0, 3, 4, 2, 2]
                .iter()
                .map(|&x| {
                    if x == 0 {
                        Cell { value: None }
                    } else {
                        Cell { value: Some(x) }
                    }
                })
                .collect(),
        );
        grid.move_cells(Direction::UP);

        assert_eq!(
            vec![4, 4, 4, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            grid.get_grid()
        );
    }

    #[test]
    fn swipe_down() {
        let mut grid = Grid::new_grid(
            4,
            vec![0, 2, 0, 0, 4, 0, 0, 0, 0, 2, 2, 0, 3, 4, 2, 2]
                .iter()
                .map(|&x| {
                    if x == 0 {
                        Cell { value: None }
                    } else {
                        Cell { value: Some(x) }
                    }
                })
                .collect(),
        );
        grid.move_cells(Direction::DOWN);

        assert_eq!(
            vec![0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 0, 0, 3, 4, 4, 2,],
            grid.get_grid()
        );
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..self.size)
                .into_iter()
                .map(|i| {
                    format!(
                        "{}\n",
                        (0..self.size)
                            .into_iter()
                            .map(move |j| {
                                let idx = self.get_index_from_coord((j, i).into());
                                let cell = &self.grid[idx];
                                if let Some(val) = cell.value {
                                    val.to_string()
                                } else {
                                    "_".to_string()
                                }
                            })
                            .collect::<String>()
                    )
                })
                .collect::<String>()
        )
    }
}
