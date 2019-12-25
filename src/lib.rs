extern crate wasm_bindgen;

use std::cmp::max;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello {}!", name));
}

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

impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        Universe {
            width,
            height,
            cells: vec![Cell::Dead; (width * height) as usize],
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<Cell> {
        if x < self.width && y < self.height {
            Some(self.cells[(y * self.width + x) as usize])
        } else {
            None
        }
    }

    pub fn toggle(&mut self, x: u32, y: u32) {
        self.cells[(y * self.width + x) as usize] = match self.get(x, y) {
            None => return,
            Some(Cell::Dead) => Cell::Alive,
            Some(Cell::Alive) => Cell::Dead,
        }
    }

    pub fn alive_around(&self, x: u32, y: u32) -> u8 {
        let mut count = 0u8;
        for ix in max(x - 1, 0)..=x+1 {
            for iy in max(y - 1, 0)..=y+1 {
                if (ix != x || iy != y) && self.get(ix, iy) == Some(Cell::Alive) {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use crate::Cell;
    use crate::Universe;

    #[test]
    fn basics() {
        let mut uni = Universe::new(3, 3);
        assert_eq!(uni.get(1, 1), Some(Cell::Dead));
        uni.toggle(1, 1);
        assert_eq!(uni.get(1, 1), Some(Cell::Alive));
        uni.toggle(100, 100);
        assert_eq!(uni.get(1, 1), Some(Cell::Alive));
    }

    #[test]
    fn game_of_life_count() {
        let mut uni = Universe::new(10, 10);
        assert_eq!(uni.alive_around(3, 6), 0);
        uni.toggle(3, 6);
        uni.toggle(4, 6);
        uni.toggle(2, 6);
        uni.toggle(3, 7);
        assert_eq!(uni.alive_around(3, 6), 3);
        assert_eq!(uni.alive_around(6, 3), 0);
        assert_eq!(uni.alive_around(100, 100), 0);
    }
}
