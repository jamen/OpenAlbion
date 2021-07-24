mod input;
mod scene;

use glam::{Quat, Vec3};
pub use input::*;
pub use scene::*;
use winit::event::VirtualKeyCode;

use std::path::PathBuf;
use std::time::Instant;

pub struct State {
    pub fable_dir: PathBuf,
    pub input: InputState,
    pub camera_position: Vec3,
    pub camera_velocity: Vec3,
    pub camera_rotation: Quat,
    pub last: Instant,
}

impl State {
    pub fn new(fable_dir: PathBuf) -> Self {
        Self {
            fable_dir,
            input: InputState::new(),
            camera_position: Vec3::new(100.0, 100.0, 100.0),
            camera_velocity: Vec3::ZERO,
            camera_rotation: Quat::IDENTITY,
            last: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        log::info!("last {:?}", self.last.elapsed());

        self.last = Instant::now();

        if self.input.keys[VirtualKeyCode::W as usize].is_some() {
            self.camera_position.x += 10.0;
        }
        if self.input.keys[VirtualKeyCode::S as usize].is_some() {
            self.camera_position.x -= 10.0;
        }

        if self.input.keys[VirtualKeyCode::A as usize].is_some() {
            self.camera_position.y += 10.0;
        }
        if self.input.keys[VirtualKeyCode::D as usize].is_some() {
            self.camera_position.y -= 10.0;
        }

        if self.input.keys[VirtualKeyCode::Space as usize].is_some() && self.input.modifiers.shift()
        {
            self.camera_position.z -= 10.0;
        } else if self.input.keys[VirtualKeyCode::Space as usize].is_some() {
            self.camera_position.z += 10.0;
        }
    }
}
