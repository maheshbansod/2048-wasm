use std::fmt::Display;

use wasm_bindgen::prelude::wasm_bindgen;

pub struct Grid {
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
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    value: Option<u32>,
}

impl Grid {
    pub fn new(size: usize) -> Self {
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
        if empty_cells.is_empty() {
            return Err("No empty cell found apparently".to_string());
        }
        let random_idx = (js_sys::Math::random() * empty_cells.len() as f64).floor() as usize;
        let cell = empty_cells[random_idx];
        let idx = self.get_index_from_coord(cell);
        self.grid[idx].value = Some(value);
        Ok(())
    }

    // returns the score of the move
    pub fn move_cells(&mut self, direction: Direction) -> u32 {
        let mut score = 0;
        let is_vertical = direction == Direction::Up || direction == Direction::Down;
        let is_down_or_right = direction == Direction::Right || direction == Direction::Down;
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

impl Default for Grid {
    fn default() -> Self {
        Self::new(4)
    }
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
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

#[cfg(test)]
mod tests {
    use super::*;

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
        grid.move_cells(Direction::Right);

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
        grid.move_cells(Direction::Left);

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
        grid.move_cells(Direction::Up);

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
        grid.move_cells(Direction::Down);

        assert_eq!(
            vec![0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 0, 0, 3, 4, 4, 2,],
            grid.get_grid()
        );
    }
}
