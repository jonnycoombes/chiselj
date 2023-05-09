use crate::{errors::ChiselResult, render::commands::CommandList};
use std::sync::mpsc::Sender;

pub mod filter;
pub mod pointers;
pub mod print;

/// An action context provides all the information and configuration needed to process an action
#[derive(Debug)]
pub struct CommandContext {
    /// The rendering pipeline
    pub render_pipeline: Sender<CommandList>,
}

impl CommandContext {
    /// Clone the render pipeline
    pub fn clone_render_pipeline(&self) -> Sender<CommandList> {
        self.render_pipeline.clone()
    }
}

/// Defines an interface for commands supported by the application
pub trait Command {
    /// Execute the action, taking in a reference to
    fn execute(&self, context: &mut CommandContext) -> ChiselResult<()>;
}
