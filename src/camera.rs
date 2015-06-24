use glium::glutin::VirtualKeyCode;
use std::sync::{Arc, Mutex};
use oxen::{Behaviour, Transform};
use oxen::Camera as OxenCamera;

pub struct Camera {
    pub camera: Arc<Mutex<OxenCamera>>,
}

impl Camera {
    pub fn new(x: f32, y: f32) -> Camera {
        let transform = Transform{x: x, y: y, scale: 1., visible: true};
        let camera = Arc::new(Mutex::new(OxenCamera{transform: transform} ));
        Camera{camera: camera}
    }
}

impl Behaviour for Camera {
    fn update(&mut self, key_pressed: &Fn(VirtualKeyCode) -> bool) {
        if key_pressed(VirtualKeyCode::W) {
            self.camera.lock().unwrap().transform.y += 0.01;
        }
        if key_pressed(VirtualKeyCode::S) {
            self.camera.lock().unwrap().transform.y -= 0.01;
        }
        if key_pressed(VirtualKeyCode::A) {
            self.camera.lock().unwrap().transform.x -= 0.01;
        }
        if key_pressed(VirtualKeyCode::D) {
            self.camera.lock().unwrap().transform.x += 0.01;
        }
    }
}

