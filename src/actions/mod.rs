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
    pub render_pipeline: Sender<CommandList>,
}

impl<'a, Args> ActionContext<'a, Args> {
    /// Clone the render pipeline
    pub fn clone_render_pipeline(&self) -> Sender<CommandList> {
        self.render_pipeline.clone()
    }
}

/// An action is
pub trait Action<Args, Output> {
    /// Execute the action, taking in a reference to
    fn execute<'a, 'b>(&mut self, context: &mut ActionContext<'a, Args>) -> ChiselResult<Output>;
}
