use std::sync::{Arc, Mutex};
use oxen::Transform;
use seeds::Seed;
use oxen::Oxen;

pub struct Cell {
    pub transform: Arc<Mutex<Transform>>,
    pub neighbours: Vec<usize>,
}

impl Cell {
    pub fn new<F>(oxen: &mut Oxen, x: i16, y: i16, seed: Seed, square_size: f32, coords_to_index: F) -> Cell
        where F : Fn((i16, i16)) -> usize {
        let cell = Cell {
            transform: Arc::new(Mutex::new(Transform::new(
                x as f32 * square_size + square_size / 2., -(y as f32 * square_size + square_size / 2.), 0.,
                0., 0., 0.,
                square_size, square_size, square_size,
                seed(x, y),
            ))),
            neighbours: [
                (x-1, y-1), (x, y-1), (x+1, y-1),
                (x-1, y  ),           (x+1, y  ),
                (x-1, y+1), (x, y+1), (x+1, y+1)
            ].iter().map(|n| coords_to_index(*n)).collect(),
        };
        oxen.attach_render_object(cell.transform.clone(), "square").unwrap();
        cell
    }

    pub fn update(&mut self, alive_neighbours: usize) {
        let mutex = self.transform.clone();
        let mut transform = mutex.lock().unwrap();
        transform.visible = match (transform.visible, alive_neighbours) {
            (false, 3) => true,
            (true, 2) => true,
            (true, 3) => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use super::Cell;
    use oxen::Transform;

    fn new_cell(visible: bool) -> Cell {
        Cell { transform: Arc::new(Mutex::new(Transform::new(0., 0., 0., 0., 0., 0., 1., 1., 1., visible))), neighbours: Vec::new()}
    }

    #[test]
    fn it_dies_if_alive_with_less_than_2_living_neighbours() {
        let mut cell = new_cell(true);
        cell.update(1);
        let mutex = cell.transform.clone();
        let t = mutex.lock().unwrap();
        assert!(t.visible == false);
    }

    #[test]
    fn it_dies_if_alive_with_more_than_3_living_neighbours() {
        let mut cell = new_cell(true);
        cell.update(4);
        let mutex = cell.transform.clone();
        let t = mutex.lock().unwrap();
        assert!(t.visible == false);
    }

    #[test]
    fn it_lives_if_alive_with_2_or_3_living_neighbours() {
        let mut cell = new_cell(true);
        cell.update(2);
        let mutex = cell.transform.clone();
        let mut t = mutex.lock().unwrap();
        assert!(t.visible == true);
        drop(t);
        cell.update(3);
        t = mutex.lock().unwrap();
        assert!(t.visible == true);
    }

    #[test]
    fn it_is_born_if_dead_with_exactly_3_living_neighbours() {
        let mut cell = new_cell(false);
        cell.update(3);
        let mutex = cell.transform.clone();
        let t = mutex.lock().unwrap();
        assert!(t.visible == true);
    }

    #[test]
    fn it_stays_dead_if_dead_with_less_than_3_living_neighbours() {
        let mut cell = new_cell(false);
        cell.update(2);
        let mutex = cell.transform.clone();
        let t = mutex.lock().unwrap();
        assert!(t.visible == false);
    }

    #[test]
    fn it_stays_dead_if_dead_with_more_than_3_living_neighbours() {
        let mut cell = new_cell(false);
        cell.update(4);
        let mutex = cell.transform.clone();
        let t = mutex.lock().unwrap();
        assert!(t.visible == false);
    }
}
