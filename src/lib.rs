mod utils;

use std::fmt;
use std::fmt::{Formatter};
use wasm_bindgen::prelude::*;
// use crate::Cell::{Alive, Dead};
use fixedbitset::FixedBitSet;
use js_sys::Math::random;

extern crate web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// #[repr(u8)]
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum Cell {
//     Dead = 0,
//     Alive = 1,
// }

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();

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

    pub fn randomize(&mut self) {
        for i in 0..self.cells.len() {
            let random_value = random();
            self.cells.set(i, if random_value < 0.5 {
                true
            } else {
                false
            });
        }
    }

    fn get_size(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn kill_cells(&mut self) {
        self.cells = FixedBitSet::with_capacity(self.get_size())
    }

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

                // log!(
                //     "cell[{}, {}] is initially {:?} and has {} live neighbors",
                //     row,
                //     col,
                //     cell,
                //     live_neighbors
                // );

                let next_cell = match (cell, live_neighbors) {
                    //Case 1: Alive, < 2 neighbors, ded
                    (true, x) if x < 2 => {
                        log!(
                            "cell[{}, {}] has died :(",
                            row,
                            col
                        );
                        false
                    },
                    //Case 2: Alive,  2-3 neighbors. survive
                    (true, 2) | (true, 3) => {
                        log!(
                            "cell[{}, {}] is born! :D",
                            row,
                            col
                        );
                        true
                    },
                    //Case 3: Alive,  > 3 neighbors, ded
                    (true, x) if x > 3 => {
                        log!(
                            "cell[{}, {}] has died :(",
                            row,
                            col
                        );
                        false
                    },
                    //Case 4: Dead, 3 neighbors, alive
                    (false, 3) => {
                        log!(
                            "cell[{}, {}] is born! :D",
                            row,
                            col
                        );
                        true
                    },
                    //else, stay same
                    (otherwise, _) => otherwise,
                };
                // log!("  it becomes {:?}", next_cell);

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

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = FixedBitSet::with_capacity((width * self.height) as usize);
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = FixedBitSet::with_capacity((self.width * height) as usize);
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx =self.get_index(row, column);
        self.cells.toggle(idx);
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

impl Universe {
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}

