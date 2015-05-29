use std::sync::{Arc, Mutex};

use oxen::Transform;

use cell::Cell;
use seeds::Seed;
use oxen::Behaviour;

pub struct Grid {
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new(seed: Seed, width: i16, height: i16, square_size: f32) -> Grid {
        let mut cells = Vec::new();

        for y in (0..height) {
            for x in (0..width) {
                cells.push(Cell {
                    transform: Arc::new(Mutex::new(Transform {
                        x: (x as f32 * square_size + square_size / 2.),
                        y: -(y as f32 * square_size + square_size / 2.),
                        visible: seed(x, y),
                        scale: square_size,
                    })),
                    neighbours: [
                        (x-1, y-1), (x, y-1), (x+1, y-1),
                        (x-1, y  ),           (x+1, y  ),
                        (x-1, y+1), (x, y+1), (x+1, y+1)
                    ].iter().map(|n| coords_to_index(*n, width, height)).collect(),
                });
            }
        }
        Grid{ cells: cells }
    }
}

impl Behaviour for Grid {
    fn update(&mut self) {
        let mut alive_neighbours = Vec::new();
        for cell in self.cells.iter() {
            alive_neighbours.push(cell.neighbours.iter().filter(|i| {
                let mutex = self.cells[**i].transform.clone();
                let t = mutex.lock().unwrap();
                t.visible
            }).count())
        }

        for (cell, cell_alive_neighbours) in self.cells.iter_mut().zip(alive_neighbours.iter()) {
            cell.update(*cell_alive_neighbours)
        }
    }
}

fn coords_to_index(coords: (i16, i16), grid_width: i16, grid_height: i16) -> usize {
    let (x, y) = coords;
    let x_wrapped = (x + grid_width) % grid_width;
    let y_wrapped = (y + grid_height) % grid_height;
    (x_wrapped + (y_wrapped * grid_width)) as usize
}

#[cfg(test)]
mod tests {
    use super::coords_to_index;

    #[test]
    fn it_returns_the_x_value_on_the_first_row() {
        assert!(coords_to_index((3, 0), 5, 3) == 3)
    }

    #[test]
    fn it_wraps_overflowing_x_values() {
        assert!(coords_to_index((6, 0), 5, 3) == 1)
    }

    #[test]
    fn it_wraps_underflowing_x_values() {
        assert!(coords_to_index((-1, 0), 5, 3) == 4)
    }

    #[test]
    fn it_adds_one_width_for_each_row() {
        assert!(coords_to_index((2, 2), 5, 3) == 12)
    }

    #[test]
    fn it_wraps_overflowing_y_values() {
        assert!(coords_to_index((1, 5), 5, 3) == 11)
    }

    #[test]
    fn it_wraps_underflowing_y_values() {
        assert!(coords_to_index((4, -2), 5, 3) == 9)
    }
}
