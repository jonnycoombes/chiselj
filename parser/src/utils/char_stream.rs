#![allow(unused_macros)]
#![allow(clippy::transmute_int_to_char)]
//! A character-oriented stream implementation that will take an underlying [std::u8] (byte) source
//! and produce a stream of decoded Unicode (UTF-8) characters
use std::borrow::Cow;
use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};
use std::io::{Bytes, Read, Take};
use std::mem::transmute;
use std::str::Utf8Error;

use crate::parser_errors::*;
use crate::parser_errors::ParserResult;
use crate::stream_error;

/// Mask for extracing 7 bits from a single byte sequence
const SINGLE_BYTE_MASK: u32 = 0b0111_1111;
/// Mask for extracting initial 5 bits within a double byte UTF-8 ssequence
const DOUBLE_BYTE_MASK: u32 = 0b0001_1111;
/// Mask for extracting initial 4 bits within a triple byte UTF-8 ssequence
const TRIPLE_BYTE_MASK: u32 = 0b0000_1111;
/// Mask for extracting initial 3 bits within a quad byte UTF-8 ssequence
const QUAD_BYTE_MASK: u32 = 0b0000_0111;
/// Mask for extracting 6 bits from following byte UTF-8 ssequences
const FOLLOWING_BYTE_MASK: u32 = 0b0011_1111;

/// Convenience macro for some bit twiddlin'
macro_rules! single_byte_sequence {
    ($byte : expr) => {
        $byte >> 7 == 0b0000_0000
    };
}

/// Convenience macro for some bit twiddlin'
macro_rules! double_byte_sequence {
    ($byte : expr) => {
        $byte >> 5 == 0b0000_0110
    };
}

/// Convenience macro for some bit twiddlin'
macro_rules! triple_byte_sequence {
    ($byte : expr) => {
        $byte >> 4 == 0b0000_1110
    };
}

/// Convenience macro for some bit twiddlin'
macro_rules! quad_byte_sequence {
    ($byte : expr) => {
        $byte >> 3 == 0b0001_1110
    };
}

#[inline(always)]
fn decode_double(a: u32, b: u32) -> u32 {
    (b & FOLLOWING_BYTE_MASK) | ((a & DOUBLE_BYTE_MASK) << 6)
}

#[inline(always)]
fn decode_triple(a: u32, b: u32, c: u32) -> u32 {
    (c & FOLLOWING_BYTE_MASK) | ((b & FOLLOWING_BYTE_MASK) << 6) | ((a & TRIPLE_BYTE_MASK) << 12)
}

#[inline(always)]
fn decode_quad(a: u32, b: u32, c: u32, d: u32) -> u32 {
    (d & FOLLOWING_BYTE_MASK)
        | ((c & FOLLOWING_BYTE_MASK) << 6)
        | ((b & FOLLOWING_BYTE_MASK) << 12)
        | ((a & QUAD_BYTE_MASK) << 18)
}

/// A character stream, which is wrapped around a given [Read] instance.
/// The lifetime of the reader instance must be at least as long as the character stream
pub struct CharStream<'a, Reader: Read + Debug> {
    /// The input stream
    input: Bytes<&'a mut Reader>,
}

impl<'a, Reader: Read + Debug> CharStream<'a, Reader> {
    /// Create a new character stream with a default buffer size
    pub fn new(r: &'a mut Reader) -> Self {
        CharStream { input: r.bytes() }
    }

    /// Attempt to advance over the next character in the underlying stream. Assumes the maximum
    /// number of unicode bytes is 4 *not* 6
    pub fn next_char(&mut self) -> ParserResult<char> {
        let leading_byte = self.next_packed_byte()?;
        unsafe {
            if single_byte_sequence!(leading_byte) {
                return Ok(transmute(leading_byte));
            }
            if double_byte_sequence!(leading_byte) {
                return Ok(transmute(decode_double(
                    leading_byte,
                    self.next_packed_byte()?,
                )));
            }
            if triple_byte_sequence!(leading_byte) {
                return Ok(transmute(decode_triple(
                    leading_byte,
                    self.next_packed_byte()?,
                    self.next_packed_byte()?,
                )));
            }
            if quad_byte_sequence!(leading_byte) {
                return Ok(transmute(decode_quad(
                    leading_byte,
                    self.next_packed_byte()?,
                    self.next_packed_byte()?,
                    self.next_packed_byte()?,
                )));
            }
        }
        stream_error!(
            ParserErrorCode::InvalidByteSequence,
            "failed to decode any valid UTF-8"
        )
    }

    /// Attempt to read a single byte from the input
    #[inline(always)]
    fn next_packed_byte(&mut self) -> ParserResult<u32> {
        match self.input.next() {
            Some(result) => match result {
                Ok(b) => Ok(b as u32),
                Err(_) => stream_error!(ParserErrorCode::StreamFailure, "failed to read next byte"),
            },
            None => stream_error!(ParserErrorCode::EndOfInput, "no more bytes available"),
        }
    }
}

impl<'a, Reader: Read + Debug> Debug for CharStream<'a, Reader> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "rdr: {:?}", self.input)
    }
}

impl<'a, Reader: Read + Debug> Iterator for CharStream<'a, Reader> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_char() {
            Ok(c) => Some(c),
            Err(_) => None,
        }
    }
}
