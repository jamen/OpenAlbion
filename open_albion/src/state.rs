use std::collections::HashMap;

use winit::event::{VirtualKeyCode,ModifiersState};
use winit::event::{Event,WindowEvent,DeviceEvent,KeyboardInput,ElementState};

#[derive(Default)]
pub struct State {
    pub keys: HashMap<VirtualKeyCode, KeyCondition>,
    pub modifiers: ModifiersState,
}

/// Whether a key press is on its first frame or its been held multiple frames.
pub enum KeyCondition {
    First,
    Held,
}

impl State {
    pub fn update(&mut self) {

    }
}

impl State {
    pub fn handle_window_event(&mut self, event: Event<'static, ()>) {
        match event {
            Event::WindowEvent { event: window_event, .. } => {
                match window_event {
                    WindowEvent::KeyboardInput {
                        input: KeyboardInput { virtual_keycode: Some(virtual_keycode), state: element_state, .. },
                        ..
                    } => {
                        match element_state {
                            ElementState::Pressed => self.on_key_pressed_or_held(virtual_keycode),
                            ElementState::Released => self.on_key_released(virtual_keycode),
                        }
                    },
                    WindowEvent::Focused(true) => self.on_focus(),
                    WindowEvent::Focused(false) => self.on_blur(),
                    WindowEvent::ModifiersChanged(modifiers) => self.on_key_modifiers_changed(modifiers),
                    _ => {}
                }
            },
            Event::DeviceEvent { event: device_event, .. } => {
                match device_event {
                    DeviceEvent::MouseMotion { delta } => self.on_mouse_motion(delta),
                    _ => {}
                }
            },
            _ => {}
        }
    }

    pub fn on_key_pressed_or_held(&mut self, key: VirtualKeyCode) {
        // println!("on_key_pressed_or_held {:?}", key);
    }

    pub fn on_key_released(&mut self, key: VirtualKeyCode) {
        // println!("on_key_released {:?}", key);
    }

    pub fn on_key_modifiers_changed(&mut self, modifiers: ModifiersState) {
        // println!("on_key_modifiers_changed {:?}", modifiers);
    }

    pub fn on_focus(&mut self) {
        // println!("on_focus");
    }

    pub fn on_blur(&mut self) {
        // println!("on_blur");
    }

    pub fn on_mouse_motion(&mut self, delta: (f64, f64)) {
        // println!("on_mouse_motion {:?}", delta);
    }
}