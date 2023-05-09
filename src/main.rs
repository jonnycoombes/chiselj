#![allow(dead_code)]

use crate::render::options::DrawOptions;
use crate::state::AppChangeState;
use clap::Parser as ClapParser;
use cli::{AppArguments, AppCommand};
use commands::{Command, CommandContext};

mod cli;
mod commands;
mod errors;
mod render;
mod sources;
mod state;
mod threads;

/// Create a new [CommandContext]
fn new_state_and_context(render_options: DrawOptions) -> (AppChangeState, CommandContext) {
    let state = AppChangeState::new(render_options);
    let context = CommandContext::new(state.get_render_pipeline());
    (state, context)
}

/// This is where the fun starts
fn main() {
    let mut exit_code = 0;
    let args = AppArguments::parse();
    let render_options = DrawOptions::default();

    match args.command {
        AppCommand::Print(cmd) => {
            let (mut state, mut context) = new_state_and_context(render_options);
            match cmd.execute(&mut context) {
                Ok(_) => (),
                Err(e) => {
                    exit_code = 1;
                    println!("💥{}", e)
                }
            }
            state.halt_renderer();
        }
        AppCommand::Filter(cmd) => {
            let (mut state, mut context) = new_state_and_context(render_options);
            match cmd.execute(&mut context) {
                Ok(_) => (),
                Err(e) => {
                    exit_code = 1;
                    println!("💥{}", e)
                }
            }
            state.halt_renderer();
        }
        AppCommand::Pointers(cmd) => {
            let (mut state, mut context) = new_state_and_context(render_options);
            match cmd.execute(&mut context) {
                Ok(_) => (),
                Err(e) => {
                    exit_code = 1;
                    println!("💥{}", e)
                }
            }
            state.halt_renderer();
        }
    }

    // return a well-behaved error code
    std::process::exit(exit_code);
}
