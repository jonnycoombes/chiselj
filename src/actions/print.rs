use super::{Action, ActionContext, ActionResult};

/// An [Action] responsible for just printing (pretty or otherwise) the input
pub struct PrintAction {}

impl Action<()> for PrintAction {
    fn execute<'a>(&mut self, context: &'a mut ActionContext) -> ActionResult<()> {
        println!("Print action being executed!");
        Ok(())
    }
}
