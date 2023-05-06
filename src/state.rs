use crate::render::commands::{CommandList, CommandListMode, PipelineCommand, StateCommand};
use crate::render::renderer::{new_renderer, RenderOptions};
use crate::state;
use crate::threads::AppThreads;
use std::sync::mpsc::Sender;

/// Struct representing the global application state
pub struct AppState {
    /// Worker threads
    threads: AppThreads,
}

impl AppState {
    /// Create a new instance of the global application state
    pub fn new(render_options: RenderOptions) -> Self {
        AppState {
            threads: AppThreads {
                renderer: new_renderer(render_options),
            },
        }
    }

    /// Get a clone of the transmitter for the rendering thread
    pub fn get_render_pipeline(&self) -> Sender<CommandList> {
        self.threads.renderer.sink.clone()
    }

    /// Halt the rendering thread by throwing it a [StateCommand::Terminate]
    pub fn halt_renderer(&mut self) {
        self.threads
            .renderer
            .sink
            .send(CommandList {
                mode: CommandListMode::Immediate,
                cmds: vec![state!(StateCommand::Terminate)],
            })
            .expect("Failed to send terminate op to renderer");
        self.threads.renderer.join();
    }
}
