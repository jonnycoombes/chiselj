use super::{Command, CommandContext};
use crate::errors::ChiselResult;
use crate::render::display_lists::{DisplayList, DisplayListCommand, DisplayListMode, Draw};
use crate::sources::{source_from_file, source_from_stdin};
use chisel_json::errors::ParserResult;
use chisel_json::events::Event;
use chisel_json::sax::Parser as SaxParser;
use clap::{Args, ValueEnum};
use std::path::PathBuf;

/// An [Command] responsible for filtering the input
#[derive(Debug, Args)]
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
    #[arg(value_enum, long, value_name = "TYPES", value_delimiter = ',')]
    pub types: Vec<PointerType>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum PointerType {
    /// object elements
    Objects,
    /// array elements
    Arrays,
    /// object member key elements
    Keys,
    /// string value elements
    Strings,
    /// numeric value elements
    Numbers,
    /// boolean value elements
    Booleans,
    /// null value elements
    Nulls,
}

impl PointersCommand {
    /// All the SAX event processing passes through here
    fn handle_sax_event(&self, evt: &Event) -> ParserResult<()> {
        match evt.pointer {
            Some(p) => println!("{}", p.to_string()),
            None => (),
        }
        Ok(())
    }
}

impl Command for PointersCommand {
    fn execute(&self, _context: &mut CommandContext) -> ChiselResult<()> {
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
        let _result = parser.parse_bytes(&buffer, &mut |evt| self.handle_sax_event(evt));

        Ok(())
    }
}
