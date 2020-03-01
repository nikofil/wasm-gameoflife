#![feature(test)]

#[cfg(not(target_os="wasi"))]
extern crate wasm_bindgen;
extern crate test;

#[cfg(not(target_os="wasi"))]
use wasm_bindgen::prelude::*;
use std::cmp::max;
use std::fmt::Display;

#[cfg_attr(not(target_os="wasi"), wasm_bindgen)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[cfg_attr(not(target_os="wasi"), wasm_bindgen)]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<u8>,
}

#[cfg_attr(not(target_os="wasi"), wasm_bindgen)]
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        Universe {
            width,
            height,
            cells: vec![0; (width * height / 8 + 1) as usize],
        }
    }

    pub fn default() -> Universe {
        Universe::default_with_size(64)
    }

    pub fn default_with_size(size: u32) -> Universe {
        let wd = size;
        let ht = size;
        let mut uni = Universe::new(wd, ht);
        for idx in 0..(wd*ht) {
            if idx % 2 == 0 || idx % 7 == 0 {
                uni.toggle(idx % wd, idx / wd);
            }
        }
        uni
    }

    pub fn get(&self, x: u32, y: u32) -> Option<Cell> {
        if x < self.width && y < self.height {
            let idx = (y * self.width + x) as usize;
            match self.cells[idx / 8] & (1 << (idx % 8)) {
                0 => Some(Cell::Dead),
                _ => Some(Cell::Alive),
            }
        } else {
            None
        }
    }

    pub fn toggle(&mut self, x: u32, y: u32) {
        let idx = (y * self.width + x) as usize;
        if self.get(x, y).is_some() {
            self.cells[idx / 8] ^= 1 << (idx % 8);
        }
    }

    pub fn alive_around(&self, x: u32, y: u32) -> u8 {
        let mut count = 0u8;
        for ix in max(x, 1)-1..=x+1 {
            for iy in max(y, 1)-1..=y+1 {
                if (ix != x || iy != y) && self.get(ix, iy) == Some(Cell::Alive) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next = Self::new(self.width, self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                match (self.get(x, y).unwrap(), self.alive_around(x, y)) {
                    (Cell::Alive, 2) | (Cell::Alive, 3) => next.toggle(x, y),
                    (Cell::Dead, 3) => next.toggle(x, y),
                    _ => {},
                }
            }
        }
        self.cells = next.cells;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u8 {
        self.cells.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", if self.get(x, y).unwrap() == Cell::Alive {
                    'x'
                } else {
                    '.'
                })?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Cell;
    use crate::Universe;
    use test::Bencher;

    #[test]
    fn test_basics() {
        let mut uni = Universe::new(3, 3);
        assert_eq!(uni.get(1, 1), Some(Cell::Dead));
        uni.toggle(1, 1);
        assert_eq!(uni.get(1, 1), Some(Cell::Alive));
        uni.toggle(100, 100);
        assert_eq!(uni.get(1, 1), Some(Cell::Alive));
    }

    #[test]
    fn test_count() {
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

    #[test]
    fn test_fmt() {
        let mut uni = Universe::new(3, 3);
        uni.toggle(1, 1);
        uni.toggle(1, 2);
        assert_eq!(uni.to_string(), "...\n.x.\n.x.\n");
    }

    #[test]
    fn test_tick() {
        let mut uni = Universe::new(3, 3);
        uni.toggle(1, 0);
        uni.toggle(1, 1);
        uni.toggle(1, 2);
        assert_eq!(uni.to_string(), ".x.\n.x.\n.x.\n");
        uni.tick();
        assert_eq!(uni.to_string(), "...\nxxx\n...\n");
        uni.tick();
        assert_eq!(uni.to_string(), ".x.\n.x.\n.x.\n");
    }

    #[bench]
    fn bench_universe_step(b: &mut Bencher) {
        let mut uni = Universe::default();
        b.iter(|| {
            uni.tick();
        });
    }
}
