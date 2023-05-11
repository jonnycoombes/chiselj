use super::{Command, CommandContext};
use crate::cl_immediate;
use crate::errors::ChiselResult;
use crate::render::display_lists::{DisplayList, DisplayListCommand, DisplayListMode, Draw};
use crate::sources::{source_from_file, source_from_stdin};
use chisel_json::errors::ParserResult;
use chisel_json::events::{Event, Match};
use chisel_json::sax::Parser as SaxParser;
use clap::{Args, ValueEnum};
use std::path::PathBuf;

// set of bits for building a filter
const OBJECT: u8 = 0b0000_0001;
const ARRAY: u8 = 0b0000_0010;
const KEY: u8 = 0b0000_0100;
const STRING: u8 = 0b0000_1000;
const NUMBER: u8 = 0b0001_0000;
const BOOLEAN: u8 = 0b0010_0000;
const NULL: u8 = 0b0100_0000;
const ALL: u8 = 0b1111_1111;

/// Map a specific [Match] to a bit which can be sieved by the currently operational filter
#[inline]
fn match_to_bit(m: &Match) -> u8 {
    match m {
        Match::StartObject => OBJECT,
        Match::StartArray => ARRAY,
        Match::ObjectKey(_) => KEY,
        Match::Float(_) | Match::Integer(_) => NUMBER,
        Match::Null => NULL,
        Match::Boolean(_) => BOOLEAN,
        Match::String(_) => STRING,
        _ => 0,
    }
}

#[inline]
fn match_to_char(m: &Match) -> char {
    match m {
        Match::StartObject => 'o',
        Match::StartArray => 'a',
        Match::ObjectKey(_) => 'k',
        Match::Float(_) | Match::Integer(_) => 'f',
        Match::Null => 'n',
        Match::Boolean(_) => 'b',
        Match::String(_) => 's',
        _ => '?',
    }
}

/// Given a list of [PointerType]s, create a bit filter
fn bit_filter(types: &Vec<PointerType>) -> u8 {
    let mut filter = 0b0000_0000;
    if types.is_empty() {
        return ALL;
    } else {
        types.iter().for_each(|t| match t {
            PointerType::Objects => filter |= OBJECT,
            PointerType::Arrays => filter |= ARRAY,
            PointerType::Keys => filter |= KEY,
            PointerType::Strings => filter |= STRING,
            PointerType::Numbers => filter |= NUMBER,
            PointerType::Booleans => filter |= BOOLEAN,
            PointerType::Nulls => filter |= NULL,
        });
    }
    filter
}

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
    fn handle_sax_event(&self, context: &CommandContext, evt: &Event) -> ParserResult<()> {
        if (match_to_bit(&evt.matched) & self.filter) > 0 {
            match evt.pointer {
                Some(p) => {
                    let _ = context.render_pipeline.send(cl_immediate!(
                        Draw::Text(format!("{}", evt.span.start.line)),
                        Draw::Char(self.delimiter),
                        Draw::Char(match_to_char(&evt.matched)),
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
