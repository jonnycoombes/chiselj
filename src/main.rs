#![allow(unused_imports)]
#![allow(dead_code)]

use clap::Parser;
use cli::Cli;
use std::io::{stdout, Write};
use std::rc::Rc;

mod cli;

/// This is where the fun starts
fn main() {
    let _cli = Cli::parse();
}
