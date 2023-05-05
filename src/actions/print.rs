use super::{Action, ActionContext, ActionResult};
use crate::cli::PrintArgs;
use crate::render::operations::{DisplayList, DisplayListMode, DrawOperation, PipelineCommand};

const GREETING: &str = "Hi from the print operation";

/// An [Action] responsible for just printing (pretty or otherwise) the input
pub struct PrintAction {}

impl Action<PrintArgs, ()> for PrintAction {
    fn execute<'a>(&mut self, context: &'a mut ActionContext<PrintArgs>) -> ActionResult<()> {
        context
            .renderer
            .send(DisplayList {
                mode: DisplayListMode::Immediate,
                ops: vec![PipelineCommand::Render(DrawOperation::Slice(GREETING))],
            })
            .expect("Failed to send to pipeline");
        Ok(())
    }
}
