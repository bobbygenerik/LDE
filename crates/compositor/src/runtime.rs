use smithay::reexports::calloop::EventLoop;
use std::time::Duration;

use crate::backend::{select_backend, BackendChoice};
use crate::output::{init_outputs, OutputState};
use crate::seat::{init_seats, SeatState};

struct CompositorState {
    outputs: Vec<OutputState>,
    seats: Vec<SeatState>,
    backend: BackendChoice,
}

pub fn run() {
    let mut event_loop: EventLoop<CompositorState> =
        EventLoop::try_new().expect("create event loop");

    let mut state = CompositorState {
        outputs: init_outputs(),
        seats: init_seats(),
        backend: select_backend(),
    };

    println!(
        "backend: {:?}, outputs: {}, seats: {}",
        match state.backend {
            BackendChoice::Winit => "winit",
            BackendChoice::Drm => "drm",
        },
        state.outputs.len(),
        state.seats.len()
    );
    for output in &state.outputs {
        println!(
            "output {}: {}x{} scale {}",
            output.name, output.width, output.height, output.scale
        );
    }
    for seat in &state.seats {
        println!("seat: {}", seat.name);
    }

    // Dispatch once to validate event loop wiring.
    let _ = event_loop.dispatch(Duration::from_millis(0), &mut state);
    // TODO: run the full event loop and integrate Smithay output + seat handling.
}
