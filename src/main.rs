#![allow(dead_code)]

use crate::render::options::RenderOptions;
use crate::state::AppState;
use actions::print::PrintAction;
use actions::{Action, ActionContext};
use clap::Parser as ClapParser;
use cli::{ActionCommand, Arguments};

mod actions;
mod cli;
mod errors;
mod render;
mod sources;
mod state;
mod threads;

/// Create a new [AppState] and [ActionContext] based on:
///
/// 1. A specialised set of command arguments
/// 2. A configuration for the renderer ([RenderOptions])
fn new_state_and_context<'a, T>(
    args: &'a T,
    render_options: RenderOptions,
) -> (AppState, ActionContext<'a, T>) {
    let state = AppState::new(render_options);
    let context = ActionContext {
        args,
        render_pipeline: state.get_render_pipeline(),
    };
    (state, context)
}

/// This is where the fun starts
fn main() {
    let mut exit_code = 0;
    let args = Arguments::parse();
    let render_options = RenderOptions::default();

    match args.command {
        ActionCommand::Print(args) => {
            let (mut state, mut context) = new_state_and_context(&args, render_options);
            let mut action = PrintAction {};
            match action.execute(&mut context) {
                Ok(_) => (),
                Err(e) => {
                    exit_code = 1;
                    println!("💥{}", e)
                }
            }
            state.halt_renderer();
        }
        ActionCommand::Filter(_args) => {
            println!("filter selected")
        }
        ActionCommand::Pointers(_args) => {
            println!("pointers selected")
        }
    }

    // return a well-behaved error code
    std::process::exit(exit_code);
}
