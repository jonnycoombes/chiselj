#![allow(dead_code)]

use crate::render::options::RenderOptions;
use crate::state::AppState;
use clap::Parser;
use cli::{AppArguments, AppCommand};
use commands::{Command, CommandContext};

mod cli;
mod commands;
mod errors;
mod render;
mod sources;
mod state;
mod threads;

/// Create a new [CommandContext] and execute the specified [Command] instance
fn execute_command(cmd: &mut impl Command) -> i32 {
    let render_options = RenderOptions::default();
    let mut state = AppState::new(render_options);
    let mut context = CommandContext::new(state.get_render_pipeline());
    match cmd.execute(&mut context) {
        Ok(_) => {
            state.halt_renderer();
            0
        }
        Err(e) => {
            println!("💥{}", e);
            state.halt_renderer();
            1
        }
    }
}

/// This is where the fun starts
fn main() {
    // parse the cl args
    let args = AppArguments::parse();

    // execute the selected command
    let exit_code = match args.command {
        AppCommand::Print(mut cmd) => execute_command(&mut cmd),
        AppCommand::Filter(mut cmd) => execute_command(&mut cmd),
        AppCommand::Pointers(mut cmd) => execute_command(&mut cmd),
    };

    // return a well-behaved error code
    std::process::exit(exit_code);
}
