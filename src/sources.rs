//! Utilities for creating different parsing sources

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

/// Create a source buffer from [stdin]
pub fn source_from_stdin(buffer: &mut Vec<u8>) -> ChiselResult<usize> {
    let mut reader = BufReader::new(stdin());
    reader
        .read_to_end(buffer)
        .or(Err(ChiselError::InvalidInput))
}
