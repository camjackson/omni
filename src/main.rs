extern crate rand;
extern crate oxen;

use std::env;
use oxen::Oxen;
use grid::Grid;

mod grid;
mod cell;
mod seeds;

fn main() {
    let seed = env::args().nth(1).map(|s|
        seeds::named(&s).expect("Invalid seed name! Valid seeds are random or gosper_glider")
    ).unwrap_or(seeds::random);

    let mut oxen = Oxen::new(1024., 768.);
    oxen.set_camera((1., -1.));

    let grid = Box::new(Grid::new(&mut oxen, seed, 128, 96, 16.));
    oxen.add_behaviour(grid);

    oxen.render_loop();
}

