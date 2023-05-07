use crate::{
    errors::{ChiselError, ChiselResult},
    render::commands::CommandList,
};
use std::sync::mpsc::Sender;
/// The default print action
pub mod print;

/// An action context provides all the information and configuration needed to process an action
#[derive(Debug)]
pub struct ActionContext<'a, Args> {
    /// The parameters for the given action
    pub args: &'a Args,

    /// The rendering pipeline
    pub pipeline: Sender<CommandList>,
}

impl<'a, Args> ActionContext<'a, Args> {
    /// Submit a sequence of rendering commands to the current rendering pipeline
    pub fn submit_render_commands(&self, commands: CommandList) -> ChiselResult<()> {
        match self.pipeline.send(commands) {
            Ok(_) => Ok(()),
            Err(_) => Err(ChiselError::RenderPipelineSendFailed),
        }
    }
}

/// An action is
pub trait Action<Args, Output> {
    /// Execute the action, taking in a reference to
    fn execute<'a, 'b>(&mut self, context: &mut ActionContext<'a, Args>) -> ChiselResult<Output>;
}
