use crate::render::display_lists::{ChangeState, DisplayList, DisplayListCommand, DisplayListMode};
use crate::render::options::DrawOptions;
use crate::render::terminal::new_renderer;
use crate::state;
use crate::threads::AppThreads;
use std::sync::mpsc::Sender;

/// Struct representing the global application state
#[derive(Debug)]
pub struct AppChangeState {
    /// Worker threads
    threads: AppThreads,
}

impl AppChangeState {
    /// Create a new instance of the global application state
    pub fn new(render_options: DrawOptions) -> Self {
        AppChangeState {
            threads: AppThreads {
                renderer: new_renderer(render_options),
            },
        }
    }

    /// Get a clone of the transmitter for the rendering thread
    pub fn get_render_pipeline(&self) -> Sender<DisplayList> {
        self.threads.renderer.sink.clone()
    }

    /// Halt the rendering thread by throwing it a [ChangeState::Terminate]
    pub fn halt_renderer(&mut self) {
        self.threads
            .renderer
            .sink
            .send(DisplayList {
                mode: DisplayListMode::Immediate,
                cmds: vec![state!(ChangeState::Terminate)],
            })
            .expect("Failed to send terminate op to renderer");
        self.threads.renderer.join();
    }
}
