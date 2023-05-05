use crate::render::operations::{DisplayList, DisplayListMode, PipelineCommand, StateOperation};
use crate::render::renderer::new_renderer;
use crate::threads::AppThreads;
use std::sync::mpsc::Sender;

/// Struct representing the global application state
pub struct AppState {
    /// Worker threads
    threads: AppThreads,
}

impl AppState {
    /// Create a new instance of the global application state
    pub fn new() -> Self {
        AppState {
            threads: AppThreads {
                renderer: new_renderer(),
            },
        }
    }

    /// Get a clone of the transmitter for the rendering thread
    pub fn get_renderer(&self) -> Sender<DisplayList> {
        self.threads.renderer.sink.clone()
    }

    /// Halt the rendering thread by throwing it a [StateOperation::Terminate]
    pub fn halt_renderer(&mut self) {
        self.threads
            .renderer
            .sink
            .send(DisplayList {
                mode: DisplayListMode::Immediate,
                ops: vec![PipelineCommand::State(StateOperation::Terminate)],
            })
            .expect("Failed to send terminate op to renderer");
        self.threads.renderer.join();
    }
}
