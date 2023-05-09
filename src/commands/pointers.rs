use super::{Command, CommandContext};
use crate::cl_immediate;
use crate::errors::{ChiselError, ChiselResult};
use crate::render::commands::{CommandList, CommandListMode, PipelineCommand, RenderCommand};
use crate::sources::{source_from_file, source_from_stdin};
use chisel_json::sax::Parser as SaxParser;
use clap::Args;
use std::collections::HashSet;
use std::path::PathBuf;

/// An [Command] responsible for filtering the input
#[derive(Debug, Args)]
pub struct PointersCommand {
    /// Source JSON file. If not specified, input is assumed to come from stdin.
    #[arg(last = true, value_name = "FILE")]
    pub file: Option<PathBuf>,
}

impl Command for PointersCommand {
    fn execute(&self, context: &mut CommandContext) -> ChiselResult<()> {
        // sort out some argument related stuff and populate the buffer
        let mut buffer: Vec<u8> = vec![];
        if let Some(path) = &self.file {
            source_from_file(path, &mut buffer)?;
        } else {
            source_from_stdin(&mut buffer)?;
        }

        let mut pointers = HashSet::new();
        let parser = SaxParser::default();
        let _result = parser.parse_bytes(&buffer, &mut |evt| {
            match evt.pointer {
                Some(p) => {
                    let s = p.as_str();
                    if !pointers.contains(&s.to_string()) {
                        let _ = context.render_pipeline.send(cl_immediate!(
                            RenderCommand::Text(format!("{}", p)),
                            RenderCommand::NewLine
                        ));
                        pointers.insert(s.to_string());
                    }
                }
                None => (),
            }
            Ok(())
        });

        Ok(())
    }
}
