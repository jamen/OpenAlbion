use std::thread;
use std::sync::mpsc;
use std::time::{Instant,Duration};

use winit::event_loop::{EventLoop,ControlFlow};
use winit::event::{Event,WindowEvent};
use winit::window::{Window,WindowBuilder};

use crate::{Renderer,State};

const FRAME_RATE: Duration = Duration::from_micros(1000000 / 120);

pub fn create_window() -> (Window, EventLoop<()>) {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Open Albion")
        .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 768.0))
        // .with_fullscreen(Some(Fullscreen::Borderless(event_loop.primary_monitor()))) // TODO: Allow windowed later.
        .with_resizable(false) // FIXME
        .with_visible(false) // NOTE: Revealed later.
        .build(&event_loop)
        .unwrap();

    (window, event_loop)
}

pub fn run(window: Window, event_loop: EventLoop<()>) -> ! {
    let (event_sender, event_recv) = mpsc::channel();

    thread::spawn(move || {
        let mut state = State::default();
        let mut renderer = smol::block_on(Renderer::create(&window));
        // let limiter = spin_sleep::SpinSleeper::default();

        state.update();
        renderer.load_resources();
        renderer.render(&state);

        window.set_visible(true);

        loop {
            // let t = Instant::now();

            while let Ok(event) = event_recv.try_recv() {
                state.handle_window_event(event);
            }

            state.update();
            renderer.render(&state);
            // TODO sound

            // if let Some(wait) = FRAME_RATE.checked_sub(t.elapsed()) {
            //     limiter.sleep(wait);
            // }
        }
    });

    event_loop.run(move |event: Event<()>, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            },
            Event::WindowEvent { event: WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size }, .. } => {
                // TODO
            },
            event => {
                match event.to_static() {
                    Some(event) => {
                        event_sender
                            .send(event)
                            .unwrap_or_else(|err| eprintln!("dropped event with error {:?}", err));
                    },
                    None => {
                        eprintln!("dropped an unknown event");
                    }
                }
            },
        }
    });
}