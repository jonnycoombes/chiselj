#![allow(dead_code)]

use crate::render::options::RenderOptions;
use crate::state::AppChangeState;
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
fn execute_command(cmd: &impl Command) -> i32 {
    let render_options = RenderOptions::default();
    let mut state = AppChangeState::new(render_options);
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
        AppCommand::Print(cmd) => execute_command(&cmd),
        AppCommand::Filter(cmd) => execute_command(&cmd),
        AppCommand::Pointers(cmd) => execute_command(&cmd),
    };

    // return a well-behaved error code
    std::process::exit(exit_code);
}
