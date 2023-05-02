#![allow(unused_imports)]
#![allow(dead_code)]

use chisel_json::errors::ParserResult;
use chisel_json::events::{Event, Match};
use chisel_json::sax::Parser as SaxParser;
use clap::Parser as ClapParser;
use cli::Arguments;
use std::io::prelude::*;
use std::io::{self, BufReader};
mod cli;

fn on_sax_event(event: &Event) -> ParserResult<()> {
    match &event.matched {
        Match::StartOfInput => (),
        Match::EndOfInput => (),
        Match::StartObject => println!("{{"),
        Match::ObjectKey(key) => print!("{} : ", key),
        Match::EndObject => println!("}}"),
        Match::StartArray => println!("["),
        Match::EndArray => println!("]"),
        Match::String(val) => println!("{},", val),
        Match::Integer(val) => println!("{},", val),
        Match::Float(val) => println!("{},", val),
        Match::Boolean(val) => println!("{},", val),
        Match::Null => todo!(),
    }
    Ok(())
}

fn process(source: &[u8]) {
    let parser = SaxParser::default();
    match parser.parse_bytes(source, &mut on_sax_event) {
        Ok(_) => std::process::exit(0),
        Err(_) => std::process::exit(1),
    }
}

/// This is where the fun starts
fn main() {
    let mut buffer: Vec<u8> = vec![];
    let _args = Arguments::parse();
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    match reader.read_to_end(&mut buffer) {
        Ok(_) => process(buffer.as_slice()),
        Err(_) => (),
    }
}
