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
    cells: Vec<Cell>
}

impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height -1, 0, 1].iter() {
            for delta_col in [self.width -1, 0, 1].iter() {
                if *delta_row == 0 && *delta_col == 0 {
                    continue;
                }

                let neigh_row = (row + *delta_row) % self.height;
                let neigh_col = (col + *delta_col) % self.width;
                let idx = self.get_index(neigh_row, neigh_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {

    }
}
