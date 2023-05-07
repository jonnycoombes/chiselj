use super::{Action, ActionContext};
use crate::cl_immediate;
use crate::cli::PrintArgs;
use crate::errors::{ChiselError, ChiselResult};
use crate::render::commands::{CommandList, CommandListMode, PipelineCommand, RenderCommand};
use crate::sources::{source_from_file, source_from_stdin};
use chisel_json::coords::Coords;
use chisel_json::dom::Parser;
use chisel_json::JsonValue;

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

        match parse_result {
            Ok(json) => {
                self.render_json(context, 0, json)?;
            }
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

impl PrintAction {
    /// Recursively render a JSON value
    fn render_json<'a>(
        &mut self,
        context: &'a mut ActionContext<PrintArgs>,
        level: u16,
        value: JsonValue,
    ) -> ChiselResult<()> {
        match value {
            JsonValue::Object(kids) => self.render_json_object(context, level, kids),
            JsonValue::Array(_) => Ok(()),
            JsonValue::String(_) => Ok(()),
            JsonValue::Float(_) => Ok(()),
            JsonValue::Integer(_) => Ok(()),
            JsonValue::Boolean(_) => Ok(()),
            JsonValue::Null => Ok(()),
        }
    }

    /// Surround an object with braces at the correct indentation level, and recursively render
    /// children at the next indentation level
    fn render_json_object<'a>(
        &mut self,
        context: &'a mut ActionContext<PrintArgs>,
        level: u16,
        kids: Vec<(String, JsonValue)>,
    ) -> ChiselResult<()> {
        // opening brace
        context.submit_render_commands(cl_immediate!(
            RenderCommand::Indent(level * context.args.indentation),
            RenderCommand::Char('{'),
            RenderCommand::NewLine
        ))?;

        // render the kids
        for (key, value) in kids {
            self.render_json_pair(context, level + 1, key, value)?;
        }

        // closing brace with optional newline
        if level != 0 {
            context.submit_render_commands(cl_immediate!(
                RenderCommand::Indent(level * context.args.indentation),
                RenderCommand::Char('}'),
                RenderCommand::NewLine
            ))
        } else {
            context.submit_render_commands(cl_immediate!(
                RenderCommand::Indent(level * context.args.indentation),
                RenderCommand::Char('}'),
            ))
        }
    }

    /// Output a KV pair from within an object
    fn render_json_pair<'a>(
        &mut self,
        context: &'a mut ActionContext<PrintArgs>,
        level: u16,
        key: String,
        value: JsonValue,
    ) -> ChiselResult<()> {
        context.submit_render_commands(cl_immediate!(
            RenderCommand::Indent(level * context.args.indentation),
            RenderCommand::Text(key),
            RenderCommand::Indent(context.args.kvspacing),
            RenderCommand::Slice(":"),
            RenderCommand::Indent(context.args.kvspacing),
            RenderCommand::NewLine
        ))?;
        Ok(())
    }
}
