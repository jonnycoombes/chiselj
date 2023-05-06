#![allow(dead_code)]

use crate::render::renderer::RenderOptions;
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
fn create_state_and_context<'a, T>(
    args: &'a T,
    render_options: RenderOptions,
) -> (AppState, ActionContext<'a, T>) {
    let state = AppState::new(render_options);
    let context = ActionContext {
        args,
        pipeline: state.get_render_pipeline(),
    };
    (state, context)
}

/// This is where the fun starts
fn main() {
    let args = Arguments::parse();

    match args.command {
        ActionCommand::Print(args) => {
            let render_options = RenderOptions { raw: false };
            let (mut state, mut context) = create_state_and_context(&args, render_options);
            let mut action = PrintAction {};
            action
                .execute(&mut context)
                .expect("Action failed to execute");
            state.halt_renderer();
        }
        ActionCommand::Filter(args) => {
            let render_options = RenderOptions { raw: false };
            let (_state, _context) = create_state_and_context(&args, render_options);
            println!("filter selected")
        }
        ActionCommand::Pointers(args) => {
            let render_options = RenderOptions { raw: false };
            let (_state, _context) = create_state_and_context(&args, render_options);
            println!("pointers selected")
        }
    }
}
