mod renderer;
mod scene;

pub use renderer::*;
pub use scene::*;

use winit::dpi::LogicalPosition;

use std::time::Instant;

use tokio::runtime::Builder as TokioRuntimeBuilder;
use winit::window::{Window, WindowBuilder};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::event::{DeviceEvent, ElementState, Event, KeyboardInput, MouseButton, ScanCode, WindowEvent};
use winit::event::{ModifiersState, VirtualKeyCode};

use crate::{Renderer, State};

const VIRTUAL_KEY_CODE_COUNT: usize = 163;

pub struct WindowSystem {
    pub window: Window,
    pub pressed: Vec<Option<Instant>>,
    pub modifiers: ModifiersState,
    pub grabbed: bool,
}

impl WindowSystem {
    pub fn create(event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new()
            .with_title("Open Albion")
            .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 768.0))
            // TODO: .with_fullscreen(Some(Fullscreen::Borderless(event_loop.primary_monitor())))
            .with_resizable(true)
            .with_visible(false)
            .build(&event_loop)
            .unwrap();

        Self {
            window,
            pressed: vec![None; VIRTUAL_KEY_CODE_COUNT],
            modifiers: ModifiersState::empty(),
            grabbed: false,
        }
    }

    pub fn key_down(&mut self, keycode: Option<VirtualKeyCode>, scancode: ScanCode) {
        if let Some(keycode) = keycode {
            if let Some(_instant) = self.pressed[keycode as usize] {
                return
            } else {
                self.pressed[keycode as usize] = Some(Instant::now());
                println!("key_down {:?} {:?}", keycode, scancode);

                if keycode == VirtualKeyCode::Escape && self.grabbed {
                    self.ungrab();
                }
            }
        }
    }

    pub fn key_up(&mut self, keycode: Option<VirtualKeyCode>, scancode: ScanCode) {
        if let Some(keycode) = keycode {
            if let Some(instant) = self.pressed[keycode as usize] {
                println!("key_up {:?} {:?} {:?}", keycode, scancode, instant.elapsed());
                self.pressed[keycode as usize] = None;
            }
        }
    }

    pub fn modifiers_changed(&mut self, modifiers: ModifiersState) {
        self.modifiers = modifiers;
    }

    pub fn mouse_down(&mut self, button: MouseButton) {
        if !self.grabbed {
            self.grab();
        }
    }

    pub fn mouse_up(&mut self, button: MouseButton) {}

    pub fn focus(&mut self) {
        // TODO: Check other platforms. Doesn't seem like this is needed on windows at least.
        // let _ = self.window.set_cursor_grab(self.grabbed);
    }

    pub fn blur(&mut self) {}

    pub fn cursor_enter(&mut self) {}

    pub fn cursor_leave(&mut self) {}

    pub fn mouse_motion(&mut self, delta: (f64, f64)) {
        if self.grabbed {
            println!("mouse_motion {:?}", delta);
        }
    }
}

impl WindowSystem {
    fn grab(&mut self) {
        let _ = self.window.set_cursor_grab(true);
        self.window.set_cursor_visible(false);
        self.grabbed = true;
    }

    fn ungrab(&mut self) {
        let _ = self.window.set_cursor_grab(false);
        self.window.set_cursor_visible(true);
        let size = self.window.inner_size();
        let pos = LogicalPosition::new(size.width as f32 / 2.0, size.height as f32 / 2.0);
        let _ = self.window.set_cursor_position(pos);
        self.grabbed = false;
    }
}

async fn start() -> ! {
    // let mut state = State::new();

    let event_loop = EventLoop::new();

    let mut window_system = WindowSystem::create(&event_loop);

    let mut renderer = Renderer::create(&window_system.window).await;

    renderer.render();

    window_system.window.set_visible(true);

    event_loop.run(move |event: Event<()>, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode, scancode, state: element_state, .. }, .. } => match element_state {
                    ElementState::Pressed => window_system.key_down(virtual_keycode, scancode),
                    ElementState::Released => window_system.key_up(virtual_keycode, scancode),
                },
                WindowEvent::MouseInput { button, state: element_state, .. } => match element_state {
                    ElementState::Pressed => window_system.mouse_up(button),
                    ElementState::Released => window_system.mouse_down(button),
                },
                WindowEvent::ModifiersChanged(modifiers) => {
                    window_system.modifiers_changed(modifiers);
                }
                WindowEvent::Focused(true) => window_system.focus(),
                WindowEvent::Focused(false) => window_system.blur(),
                WindowEvent::CursorEntered { .. } => window_system.cursor_enter(),
                WindowEvent::CursorLeft { .. } => window_system.cursor_leave(),
                WindowEvent::Resized(size) => renderer.resize(size.width, size.height),
                // TODO
                // WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size } => { },
                WindowEvent::CloseRequested => {
                    // self.exit();
                    *control_flow = ControlFlow::Exit;
                },
                _ => {}
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => window_system.mouse_motion(delta),
                _ => {}
            },
            Event::MainEventsCleared => {
                renderer.render();
            }
            _ => {}
        }
    })
}

fn main() {
    TokioRuntimeBuilder::new_multi_thread()
        .build()
        .unwrap()
        .block_on(async { start().await });
}
