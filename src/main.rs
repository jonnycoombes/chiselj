#![allow(dead_code)]

use actions::print::PrintAction;
use actions::{Action, ActionContext};
use clap::Parser as ClapParser;
use cli::{ActionCommand, Arguments};
use core::time;

use crate::state::AppState;
mod actions;
mod cli;
mod render;
mod state;
mod threads;

/// This is where the fun starts
fn main() {
    let args = Arguments::parse();
    let mut state = AppState::new();

    match args.command {
        ActionCommand::Print(args) => {
            let mut context = ActionContext {
                args: &args,
                renderer: state.get_renderer(),
            };
            let mut action = PrintAction {};
            action
                .execute(&mut context)
                .expect("Action failed to execute");
        }
        ActionCommand::Filter(_args) => {
            println!("filter selected")
        }
    }

    std::thread::sleep(time::Duration::from_millis(5000));
    println!("Waiting for app threads to clear up and join...");
    state.halt_renderer();
}
