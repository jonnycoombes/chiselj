use super::{Action, ActionContext};
use crate::cl_immediate;
use crate::cli::PrintArgs;
use crate::errors::ChiselResult;
use crate::render::commands::{CommandList, CommandListMode, PipelineCommand, RenderCommand};
use crate::sources::{source_from_file, source_from_stdin};
use chisel_json::coords::Coords;
use chisel_json::dom::Parser;

/// An [Action] responsible for just printing (pretty or otherwise) the input
pub struct PrintAction {}

impl Action<PrintArgs, ()> for PrintAction {
    /// Execute the print action
    fn execute<'a>(&mut self, context: &'a mut ActionContext<PrintArgs>) -> ChiselResult<()> {
        // sort out some argument related stuff and populate the buffer
        let args = context.args;
        let mut buffer: Vec<u8> = vec![];
        if let Some(path) = &args.file {
            source_from_file(path, &mut buffer)?;
        } else {
            source_from_stdin(&mut buffer)?;
        }

        // grab a DOM parser and build ourselves some JSON
        let parser = Parser::default();
        let parse_result = parser.parse_bytes(&buffer);

        // render and recurse
        match parse_result {
            Ok(json) => match json {
                chisel_json::JsonValue::Object(_) => {
                    context.submit_render_commands(cl_immediate!(
                        RenderCommand::Char('{'),
                        RenderCommand::NewLine
                    ))?;

                    context.submit_render_commands(cl_immediate!(
                        RenderCommand::Char('}'),
                        RenderCommand::NewLine
                    ))?;
                }
                chisel_json::JsonValue::Array(_) => {
                    context.submit_render_commands(cl_immediate!(RenderCommand::Char('[')))?;
                    context.submit_render_commands(cl_immediate!(RenderCommand::Char(']')))?;
                }
                chisel_json::JsonValue::String(_) => todo!(),
                chisel_json::JsonValue::Float(_) => todo!(),
                chisel_json::JsonValue::Integer(_) => todo!(),
                chisel_json::JsonValue::Boolean(_) => todo!(),
                chisel_json::JsonValue::Null => todo!(),
            },
            Err(err) => {
                eprintln!("Parse failed!");
                eprintln!("\tFailed at stage: {}", err.source);
                eprintln!("\tError reported: {}", err.details);
                eprintln!(
                    "\tInput coords: {}",
                    err.coords.unwrap_or(Coords::default())
                );
            }
        };
        Ok(())
    }
}
