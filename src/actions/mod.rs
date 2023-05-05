use crate::render::operations::DisplayList;
use std::sync::mpsc::Sender;
/// The default print action
pub mod print;

/// [Action]-specific [Result] type
pub type ActionResult<T> = Result<T, ActionError>;

/// Enumeration of possible [Action] errors
#[derive(Debug)]
pub enum ActionError {
    InvalidInput,
}

/// An action context provides all the information and configuration needed to process an action
#[derive(Debug)]
pub struct ActionContext<'a, Args> {
    /// The parameters for the given action
    pub args: &'a Args,

    /// The renderer
    pub renderer: Sender<DisplayList>,
}

/// An action is
pub trait Action<Args, Output> {
    /// Execute the action, taking in a reference to
    fn execute<'a, 'b>(&mut self, context: &mut ActionContext<'a, Args>) -> ActionResult<Output>;
}
