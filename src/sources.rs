//! Utilities for creating different parsing sources

use atty::Stream;

use crate::errors::{ChiselError, ChiselResult};
use std::fs::File;
use std::io::{stdin, BufReader, Read};
use std::path::Path;

/// Create a source buffer from something that smells like a [Path]
pub fn source_from_file<PathLike: AsRef<Path>>(
    path: PathLike,
    buffer: &mut Vec<u8>,
) -> ChiselResult<usize> {
    match File::open(&path) {
        Ok(f) => {
            let mut reader = BufReader::new(&f);
            reader.read_to_end(buffer).or(Err(ChiselError::InvalidFile))
        }
        Err(err) => {
            eprintln!("{}", err);
            Err(ChiselError::InvalidFile)
        }
    }
}

/// Create a source buffer from [stdin], but only if we're not running in a TTY so that we can be
/// *reasonably* confident that we've got something coming down the pipe
pub fn source_from_stdin(buffer: &mut Vec<u8>) -> ChiselResult<usize> {
    if atty::is(Stream::Stdin) {
        return Err(ChiselError::NoPipedInput);
    }

    let mut reader = BufReader::new(stdin());
    reader
        .read_to_end(buffer)
        .or(Err(ChiselError::InvalidInput))
}
