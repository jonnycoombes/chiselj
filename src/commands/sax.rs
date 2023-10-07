//! All utility and useful functions relating to SAX-based parsing should go in here

use chisel_json::parsers::sax_events::Match;
use clap::ValueEnum;

// set of bits for building a filter

/// A pointer to an object
const OBJECT: u8 = 0b0000_0001;
/// A pointer to an array
const ARRAY: u8 = 0b0000_0010;
/// A pointer to a key
const KEY: u8 = 0b0000_0100;
/// A pointer to a string value
const STRING: u8 = 0b0000_1000;
/// A pointer to a float value
const FLOAT: u8 = 0b0001_0000;
/// A pointer to an integer value;
const INTEGER: u8 = 0b0010_0000;
/// A pointer to a boolean value
const BOOLEAN: u8 = 0b0100_0000;
/// A pointer to a null value
const NULL: u8 = 0b1000_0000;
/// A pointer to anything
const ALL: u8 = 0b1111_1111;

/// An internal enumeration for designating a specific *type* of JSON pointer - e.g. what kind of JSON element does a
/// specific pointer point to?
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum PointerType {
    /// object elements (o)
    Objects,
    /// array elements (a)
    Arrays,
    /// object member key elements (k)
    Keys,
    /// string value elements (s)
    Strings,
    /// float value elements (f)
    Floats,
    /// integer value elements (i)
    Integers,
    /// boolean value elements (b)
    Booleans,
    /// null value elements (n)
    Nulls,
}

/// Converts a specific [Match] emitted by the SAX parser into a bit string that is suitable for use in filtering
#[inline]
pub(crate) fn matched_to_bit(m: &Match) -> u8 {
    match m {
        Match::StartObject => OBJECT,
        Match::EndObject => OBJECT,
        Match::StartArray => ARRAY,
        Match::EndArray => ARRAY,
        Match::ObjectKey(_) => KEY,
        Match::Float(_) => FLOAT,
        Match::Integer(_) => INTEGER,
        Match::Null => NULL,
        Match::Boolean(_) => BOOLEAN,
        Match::String(_) => STRING,
        _ => 0,
    }
}

/// Converts a specific [Match] emitted by the SAX parser into a single `char` that can be output as part of a
/// specific command execution
#[inline]
pub(crate) fn matched_to_char(m: &Match) -> &'static str {
    match m {
        Match::StartObject => "start_obj",
        Match::EndObject => "end_obj",
        Match::StartArray => "start_arr",
        Match::EndArray => "end_arr",
        Match::ObjectKey(_) => "key",
        Match::Float(_) => "float",
        Match::Integer(_) => "int",
        Match::Null => "null",
        Match::Boolean(_) => "bool",
        Match::String(_) => "str",
        _ => "?",
    }
}

/// Given a list of [PointerType]s, create a bit filter based on the `matched_to_bit` function
pub(crate) fn bit_filter(types: &Vec<PointerType>) -> u8 {
    let mut filter = 0b0000_0000;
    if types.is_empty() {
        return ALL;
    } else {
        types.iter().for_each(|t| match t {
            PointerType::Objects => filter |= OBJECT,
            PointerType::Arrays => filter |= ARRAY,
            PointerType::Keys => filter |= KEY,
            PointerType::Strings => filter |= STRING,
            PointerType::Floats => filter |= FLOAT,
            PointerType::Integers => filter |= INTEGER,
            PointerType::Booleans => filter |= BOOLEAN,
            PointerType::Nulls => filter |= NULL,
        });
    }
    filter
}
