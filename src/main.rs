extern crate rand;
extern crate oxen;
extern crate glium;

mod grid;
mod cell;
mod seeds;
mod wasd_plane_camera;

use std::env;
use std::sync::{Arc, Mutex};
use oxen::Oxen;
use oxen::Transform;

use grid::Grid;
use wasd_plane_camera::WasdPlaneCamera;

fn main() {
    let seed = env::args().nth(1).map(|s|
        seeds::named(&s).expect("Invalid seed name! Valid seeds are random or gosper_glider")
    ).unwrap_or(seeds::random);

    let camera = Box::new(WasdPlaneCamera::new(1., -1., 1024., 768.));
    let mut oxen = Oxen::new(1024., 768., camera.oxen_camera.clone());

    let grid = Box::new(Grid::new(&mut oxen, seed, 128, 96, 16.));
    oxen.add_behaviour(grid);
    oxen.add_behaviour(camera);

    let cube = Transform::new(
        0., 0., 500.,
        0., 0., 0.,
        300., 300., 300.,
        true
    );
    oxen.attach_render_object(Arc::new(Mutex::new(cube)), "cube").unwrap();

    oxen.render_loop();
}

