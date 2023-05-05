use super::{Action, ActionContext, ActionResult};
use crate::cli::PrintArgs;
use crate::render::pipeline::{DisplayList, DisplayListMode, PipelineCommand};

const GREETING: &str = "Hi from the print operation";

/// An [Action] responsible for just printing (pretty or otherwise) the input
pub struct PrintAction {}

impl Action<PrintArgs, ()> for PrintAction {
    fn execute<'a>(&mut self, context: &'a mut ActionContext<PrintArgs>) -> ActionResult<()> {
        context
            .pipeline
            .send(DisplayList {
                mode: DisplayListMode::Immediate,
                ops: vec![PipelineCommand::Render(
                    crate::render::pipeline::DrawOperation::Slice(GREETING),
                )],
            })
            .expect("Failed to send to pipeline");
        Ok(())
    }
}
