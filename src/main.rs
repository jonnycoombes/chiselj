#![allow(unused_imports)]
#![allow(dead_code)]
#![feature(test)]
extern crate bitflags;
extern crate log;

use std::io::{stdout, Write};
use std::rc::Rc;

use crate::ui::render::opcodes::{OpCode, OpCodePtr};

mod ui;

/// Initialise logging based on the current environment
fn init_logging() {
    env_logger::try_init().unwrap();
}

/// This is where the fun starts
fn main() {
    init_logging();
}
