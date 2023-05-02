use std::io::BufRead;

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
pub struct ActionContext<'a> {
    /// A [BufRead] containing the source bytes for the action. (Should be JSON-like).
    pub input: &'a dyn BufRead,
}

/// An action is something that processes the input, based on a given configuration
pub trait Action<T> {
    /// Execute the action, taking in a reference to
    fn execute<'a>(&mut self, context: &mut ActionContext<'a>) -> ActionResult<T>;
}
