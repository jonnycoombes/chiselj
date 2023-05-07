use super::{Action, ActionContext};
use crate::cli::PrintArgs;
use crate::errors::ChiselResult;
use crate::render::commands::{CommandList, CommandListMode, PipelineCommand, RenderCommand};

const GREETING: &str = "Hi from the print operation";

/// An [Action] responsible for just printing (pretty or otherwise) the input
pub struct PrintAction {}

impl Action<PrintArgs, ()> for PrintAction {
    fn execute<'a>(&mut self, context: &'a mut ActionContext<PrintArgs>) -> ChiselResult<()> {
        Ok(())
    }
}
