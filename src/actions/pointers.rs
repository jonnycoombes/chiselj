use super::Action;
use crate::cl_immediate;
use crate::cli::PointersArgs;
use crate::errors::{ChiselError, ChiselResult};
use crate::render::commands::{CommandList, CommandListMode, PipelineCommand, RenderCommand};
use crate::sources::{source_from_file, source_from_stdin};
use chisel_json::sax::Parser;
use std::collections::HashSet;

/// An [Action] responsible for filtering the input
pub struct PointersAction {
    /// Temporary stash of previously seen pointers
    pointers: HashSet<String>,
}

impl Default for PointersAction {
    fn default() -> Self {
        Self {
            pointers: Default::default(),
        }
    }
}

impl Action<PointersArgs, ()> for PointersAction {
    fn execute<'a, 'b>(
        &mut self,
        context: &mut super::ActionContext<'a, PointersArgs>,
    ) -> ChiselResult<()> {
        // sort out some argument related stuff and populate the buffer
        let args = context.args;
        let mut buffer: Vec<u8> = vec![];
        if let Some(path) = &args.file {
            source_from_file(path, &mut buffer)?;
        } else {
            source_from_stdin(&mut buffer)?;
        }

        let parser = Parser::default();
        let _result = parser.parse_bytes(&buffer, &mut |evt| {
            match evt.pointer {
                Some(p) => {
                    let s = p.as_str();
                    if !self.pointers.contains(&s.to_string()) {
                        let _ = context.render_pipeline.send(cl_immediate!(
                            RenderCommand::Text(format!("{}", p)),
                            RenderCommand::NewLine
                        ));
                        self.pointers.insert(s.to_string());
                    }
                }
                None => (),
            }
            Ok(())
        });

        Ok(())
    }
}
