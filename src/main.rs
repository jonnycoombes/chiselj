#![allow(dead_code)]

use actions::print::PrintAction;
use actions::{Action, ActionContext};
use clap::Parser as ClapParser;
use cli::{ActionCommand, Arguments};

use crate::render::renderer::RenderOptions;
use crate::state::AppState;
mod actions;
mod cli;
mod render;
mod state;
mod threads;

/// Create a new [AppState] and [ActionContext] based on non-raw rendering
fn create_raw_state_context<'a, T>(args: &'a T) -> (AppState, ActionContext<'a, T>) {
    let render_options = RenderOptions { raw: false };
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
            let (mut state, mut context) = create_raw_state_context(&args);
            let mut action = PrintAction {};
            action
                .execute(&mut context)
                .expect("Action failed to execute");
            state.halt_renderer();
        }
        ActionCommand::Filter(args) => {
            let (_state, _context) = create_raw_state_context(&args);
            println!("filter selected")
        }
    }
}
