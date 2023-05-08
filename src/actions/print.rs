use super::{Action, ActionContext};
use crate::cli::PrintArgs;
use crate::errors::ChiselResult;
use crate::render::pretty_printer::{PrettyPrintFormatOptions, PrettyPrinter};
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

        match parse_result {
            Ok(json) => {
                // extract the formatting options from the context args
                let options = PrettyPrintFormatOptions {
                    indent: context.args.indent,
                    kvpadding: context.args.kvpadding,
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

impl PrintAction {}
