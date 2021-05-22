mod renderer;
mod state;
mod view;

pub use renderer::*;
pub use state::*;
pub use view::*;

use winit::event_loop::{EventLoop,ControlFlow};
use winit::event::{Event,WindowEvent};
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Open Albion")
        .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 768.0))
        // TODO: .with_fullscreen(Some(Fullscreen::Borderless(event_loop.primary_monitor())))
        .with_resizable(true)
        .with_visible(false)
        .build(&event_loop)
        .unwrap();

    let mut state = State::new();

    let mut renderer = smol::block_on(Renderer::create(&window));

    state.update();

    renderer.render(&state);

    window.set_visible(true);

    event_loop.run(move |event: Event<()>, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // NOTE: Might need to clone?
        match event.to_static() {
            Some(event) => {
                state.handle_window_event(&event);

                match event {
                    Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                        *control_flow = ControlFlow::Exit;
                    },
                    // TODO: I think this is for when monitor resolution changes. Currently I don't use a scale factor, so this probably breaks on 4K.
                    // Event::WindowEvent { event: WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size }, .. } => {},
                    Event::WindowEvent { event: WindowEvent::Resized(_), .. } => {
                        renderer.update_swap_chain(&window);
                    }
                    Event::MainEventsCleared => {
                        state.update();
                        renderer.render(&state);
                    },
                    _ => {},
                }
            },
            None => {
                eprintln!("event dropped because event.to_static() failed.");
            }
        }
    });
}