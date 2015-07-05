use glium::glutin::VirtualKeyCode;
use std::sync::{Arc, Mutex};
use oxen::{Behaviour, Transform};
use oxen::Camera as OxenCamera;

pub struct WasdPlaneCamera {
    pub oxen_camera: Arc<Mutex<OxenCamera>>,
}

impl WasdPlaneCamera {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> WasdPlaneCamera {
        let transform = Transform::new(
            x, y, 0.,
            0., 0., 0.,
            width, height, 1.,
            true,
        );
        let camera = Arc::new(Mutex::new(OxenCamera{transform: transform}));
        WasdPlaneCamera{oxen_camera: camera}
    }
}

impl Behaviour for WasdPlaneCamera {
    fn update(&mut self, key_pressed: &Fn(VirtualKeyCode) -> bool) {
        if key_pressed(VirtualKeyCode::W) {
            self.oxen_camera.lock().unwrap().transform.translate(0., 0.01, 0.);
        }
        if key_pressed(VirtualKeyCode::S) {
            self.oxen_camera.lock().unwrap().transform.translate(0., -0.01, 0.);
        }
        if key_pressed(VirtualKeyCode::A) {
            self.oxen_camera.lock().unwrap().transform.translate(-0.01, 0., 0.);
        }
        if key_pressed(VirtualKeyCode::D) {
            self.oxen_camera.lock().unwrap().transform.translate(0.01, 0., 0.);
        }
    }
}

