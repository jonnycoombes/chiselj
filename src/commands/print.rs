use std::path::PathBuf;

use super::{Command, CommandContext};
use crate::errors::ChiselResult;
use crate::render::pretty_printer::{FormatOptions, PrettyPrinter};
use crate::sources::{source_from_file, source_from_stdin};
use chisel_json::coords::Coords;
use chisel_json::dom::Parser;
use clap::Args;

/// An [Action] responsible for just printing (pretty or otherwise) the input
#[derive(Debug, Args)]
#[command()]
pub struct PrintCommand {
    /// Source JSON file. If not specified, input is assumed to come from stdin.
    #[arg(last = true, value_name = "FILE")]
    pub file: Option<PathBuf>,

    /// Indent space count
    ///
    /// Object keys and array values are idented by this amount plus the parent identation amount
    #[arg(short, long, value_name = "n", default_value = "2")]
    pub indent: u16,

    /// KV padding count
    ///
    /// The number of spaces added to each side of the ":" character in a <key> : <value> pair
    #[arg(short, long, value_name = "n", default_value = "1")]
    pub kvpadding: u16,
}

impl Command for PrintCommand {
    /// Execute the print action
    fn execute(&self, context: &mut CommandContext) -> ChiselResult<()> {
        // sort out some argument related stuff and populate the buffer
        let mut buffer: Vec<u8> = vec![];
        if let Some(path) = &self.file {
            source_from_file(path, &mut buffer)?;
        } else {
            source_from_stdin(&mut buffer)?;
        }

        // grab a DOM parser and build ourselves some JSON
        let parser = Parser::default();
        let parse_result = parser.parse_bytes(&buffer);

        match parse_result {
            Ok(json) => {
                // extract the formatting options from the context args
                let options = FormatOptions {
                    indent: self.indent,
                    kvpadding: self.kvpadding,
                };

                // boof it out to the printer
                let printer = PrettyPrinter::new(context.clone_render_pipeline(), options);
                printer.render_json(json)?
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

impl PrintCommand {}
