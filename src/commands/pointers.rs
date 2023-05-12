use super::sax::{bit_filter, matched_to_bit, matched_to_char, PointerType};
use super::{Command, CommandContext};
use crate::cl_immediate;
use crate::errors::ChiselResult;
use crate::render::display_lists::{DisplayList, DisplayListCommand, DisplayListMode, Draw};
use crate::sources::{source_from_file, source_from_stdin};
use chisel_json::errors::ParserResult;
use chisel_json::events::Event;
use chisel_json::sax::Parser as SaxParser;
use clap::Args;
use std::path::PathBuf;

/// An [Command] responsible for filtering the input
#[derive(Debug, Args)]
#[command(about = "Does awesome things", long_about = None)]
pub struct PointersCommand {
    /// Source JSON file.
    ///
    /// If not specified, input is assumed to come from stdin.
    #[arg(last = true, value_name = "FILE")]
    pub file: Option<PathBuf>,

    /// Pointer types.
    ///
    /// If specified, only the JSON pointers corresponding to these specific element
    /// types will be emitted
    #[arg(value_enum, short, long, value_name = "TYPES", value_delimiter = ',')]
    pub types: Vec<PointerType>,

    /// Delimiter
    ///
    /// The delimiter to be used in order to separate type codes from pointer values
    #[arg(short, long, default_value = ":")]
    pub delimiter: char,

    /// The currently operating filter, will default to [ALL]
    #[clap(skip)]
    pub filter: u8,
}

impl PointersCommand {
    /// All the SAX event processing passes through here
    fn handle_sax_event(&self, context: &CommandContext, evt: &Event) -> ParserResult<()> {
        if (matched_to_bit(&evt.matched) & self.filter) > 0 {
            match evt.pointer {
                Some(p) => {
                    let _ = context.render_pipeline.send(cl_immediate!(
                        Draw::Text(format!("{}", evt.span.start.line)),
                        Draw::Char(self.delimiter),
                        Draw::Text(format!("{}", evt.span.start.column)),
                        Draw::Char(self.delimiter),
                        Draw::Char(matched_to_char(&evt.matched)),
                        Draw::Char(self.delimiter),
                        Draw::Text(p.to_string()),
                        Draw::NewLine
                    ));
                }
                None => (),
            }
        }
        Ok(())
    }
}

impl Command for PointersCommand {
    fn execute(&mut self, context: &mut CommandContext) -> ChiselResult<()> {
        // create the bit filter to be used as we filter SAX events
        self.filter = bit_filter(&self.types);

        // sort out some argument related stuff and populate the buffer
        let mut buffer: Vec<u8> = vec![];
        if let Some(path) = &self.file {
            source_from_file(path, &mut buffer)?;
        } else {
            source_from_stdin(&mut buffer)?;
        }

        // instantiate a SAX parser instance and process the input, by delegating
        // to the `handle_sax_event` associated function
        let parser = SaxParser::default();
        let _result = parser.parse_bytes(&buffer, &mut |evt| self.handle_sax_event(context, evt));

        Ok(())
    }
}
