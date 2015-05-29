use std::sync::{Arc, Mutex};
use oxen::Transform;

pub struct Cell {
    pub transform: Arc<Mutex<Transform>>,
    pub neighbours: Vec<usize>,
}

impl Cell {
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
    use super::Cell;
    use oxen::Transform;

    fn new_cell(visible: bool) -> Cell {
        Cell { transform: Transform{x: 0., y: 0., visible: visible, scale: 1., }, neighbours: Vec::new()}
    }

    #[test]
    fn it_dies_if_alive_with_less_than_2_living_neighbours() {
        let mut cell = new_cell(true);
        cell.update(1);
        assert!(cell.transform.visible == false);
    }

    #[test]
    fn it_dies_if_alive_with_more_than_3_living_neighbours() {
        let mut cell = new_cell(true);
        cell.update(4);
        assert!(cell.transform.visible == false);
    }

    #[test]
    fn it_lives_if_alive_with_2_or_3_living_neighbours() {
        let mut cell = new_cell(true);
        cell.update(2);
        assert!(cell.transform.visible == true);
        cell.update(3);
        assert!(cell.transform.visible == true);
    }

    #[test]
    fn it_is_born_if_dead_with_exactly_3_living_neighbours() {
        let mut cell = new_cell(false);
        cell.update(3);
        assert!(cell.transform.visible == true);
    }

    #[test]
    fn it_stays_dead_if_dead_with_less_than_3_living_neighbours() {
        let mut cell = new_cell(false);
        cell.update(2);
        assert!(cell.transform.visible == false);
    }

    #[test]
    fn it_stays_dead_if_dead_with_more_than_3_living_neighbours() {
        let mut cell = new_cell(false);
        cell.update(4);
        assert!(cell.transform.visible == false);
    }
}
