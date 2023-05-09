use crate::{errors::ChiselResult, render::commands::DisplayList};
use std::sync::mpsc::Sender;

pub mod filter;
pub mod pointers;
pub mod print;

/// An action context provides all the information and configuration needed to process an action
#[derive(Debug)]
pub struct CommandContext {
    /// The rendering pipeline
    render_pipeline: Sender<DisplayList>,
}

impl CommandContext {
    pub fn new(render_pipeline: Sender<DisplayList>) -> Self {
        CommandContext {
            render_pipeline: render_pipeline.clone(),
        }
    }

    /// Clone the render pipeline
    pub fn clone_render_pipeline(&self) -> Sender<DisplayList> {
        self.render_pipeline.clone()
    }
}

/// Defines an interface for commands supported by the application
pub trait Command {
    /// Execute the action, taking in a reference to
    fn execute(&self, context: &mut CommandContext) -> ChiselResult<()>;
}
