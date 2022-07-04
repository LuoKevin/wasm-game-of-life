mod utils;

use std::fmt;
use std::fmt::{Formatter, write};
use wasm_bindgen::prelude::*;
use crate::Cell::{Alive, Dead};
use rand::prelude::*;
use fixedbitset::FixedBitSet;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width: u32 = 64;
        let height: u32 = 64;
        let size = (width * height) as usize;
        let mut bits = FixedBitSet::with_capacity(size);

        for i in 0..size {
            bits.set(i, i % 2 == 0 || i % 7 == 0);
        }

        Universe {
            width,
            height,
            cells: bits,
        }
    }

    // pub fn new_random() -> Universe {
    //     let width = 64;
    //     let height = 64;
    //     let cells = (0..width * height)
    //         .map(|i| {
    //             if random::<bool>() {
    //                 Alive
    //             } else {
    //                 Dead
    //             }
    //         })
    //         .collect();
    //
    //     Universe {
    //         width,
    //         height,
    //         cells,
    //     }
    // }
    //
    // pub fn new_spaceship() -> Universe {
    //     let width = 64;
    //     let height = 64;
    //     /** spaceship:
    //                 01001
    //                 10000
    //                 10001
    //                 11110
    //
    //      */
    //
    //     let row_size = width as usize;
    //
    //     let offset: usize = 64 * 4 + 30;
    //
    //     let mut spaceship = vec![Dead; 64 * 64];
    //     spaceship[offset + 1] = Alive;
    //     spaceship[offset + 4] = Alive;
    //     spaceship[offset + row_size * 1] = Alive;
    //     spaceship[offset + row_size * 2] = Alive;
    //     spaceship[offset + row_size * 2 + 4] = Alive;
    //     spaceship[offset + row_size * 3 + 0] = Alive;
    //     spaceship[offset + row_size * 3 + 1] = Alive;
    //     spaceship[offset + row_size * 3 + 2] = Alive;
    //     spaceship[offset + row_size * 3 + 3] = Alive;
    //
    //     Universe {
    //         width,
    //         height,
    //         cells: spaceship,
    //     }
    // }
    //
    // pub fn new_glider() -> Universe {
    //     let width = 64;
    //     let height = 64;
    //     /** spaceship:
    //                        01001
    //                        10000
    //                        10001
    //                        11110
    //
    //      */
    //
    //     let row_size = width as usize;
    //
    //     let offset: usize = 64 * 4 + 30;
    //
    //     let mut spaceship = vec![Dead; 64 * 64];
    //     spaceship[offset + 1] = Alive;
    //     spaceship[offset + row_size * 1 + 2] = Alive;
    //     spaceship[offset + row_size * 2 + 0] = Alive;
    //     spaceship[offset + row_size * 2 + 1] = Alive;
    //     spaceship[offset + row_size * 2 + 2] = Alive;
    //
    //     Universe {
    //         width,
    //         height,
    //         cells: spaceship,
    //     }
    // }

    pub fn render(&self) -> String {
        self.to_string()
    }


    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for d_row in [self.height - 1, 0, 1].iter().cloned() {
            for d_column in [self.width - 1, 0, 1].iter().cloned() {
                if d_row == 0 && d_column == 0 {
                    continue;
                }
                let neighbor_row = (row + d_row) % self.height;
                let neighbor_col = (column + d_column) % self.height;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += if self.cells[idx] {
                    1
                } else {
                    0
                }
            }
        }
        count
    }

    //Pub method, exported to JS
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    //Case 1: Alive, < 2 neighbors, ded
                    (true, x) if x < 2 => false,
                    //Case 2: Alive,  2-3 neighbors. survive
                    (true, 2) | (true, 3) => true,
                    //Case 3: Alive,  > 3 neighbors, ded
                    (true, x) if x > 3 => false,
                    //Case 4: Dead, 3 neighbors, alive
                    (false, 3) => true,
                    //else, stay same
                    (otherwise, _) => otherwise,
                };
                next.set(idx, next_cell);
            }
        }
        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == 0 { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

