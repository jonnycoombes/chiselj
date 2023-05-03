use crate::cli::PrintArgs;

use super::{Action, ActionContext, ActionResult};

/// An [Action] responsible for just printing (pretty or otherwise) the input
pub struct PrintAction {}

impl Action<PrintArgs, ()> for PrintAction {
    fn execute<'a>(&mut self, context: &'a mut ActionContext<PrintArgs>) -> ActionResult<()> {
        println!("Print action being executed! {:?}", context);
        Ok(())
    }
}
