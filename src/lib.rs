mod utils;

use core::fmt;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        // TODO: Randomize this!
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    // Get the index of a particular cell
    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        // Row, Col is the current cell
        // 3, 4
        // self.height = 5 [4, 0, 1] d_row
        // self.width = 5 [4, 0, 1] d_col
        // n_row = (3 + 4) % 5 = 2
        // n_col = (4 + 4) % 5 = 3
        // 2, 3 are valid neighbours
        let mut count = 0;
        // iterating through all the neigburing cells for our current cell
        // then just 
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                // This will be the case where the current node is located
                // so just skip this case
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                // Gettting the index of this neighbour
                let idx = self.get_index(neighbor_row, neighbor_col);
                // Just adding the value to the total sum
                // if it is 0, sum will remain unaffected otherwise increment 1
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        // For next state
        let mut next = self.cells.clone();


        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let neighbour = self.live_neighbor_count(row, col);

                // Rules for the game
                let next_cell = match(cell, neighbour) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Dead, 3)  => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3)  => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
                
                // Updating the state
                next[idx] = next_cell
            }
        }
        self.cells = next
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_cell(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = match cell {
                    Cell::Alive => '◼',
                    Cell::Dead => '◻',
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}