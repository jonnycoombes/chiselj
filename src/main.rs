#![allow(dead_code)]

use core::time;

use actions::print::PrintAction;
use actions::{Action, ActionContext};
use clap::Parser as ClapParser;
use cli::{ActionCommand, Arguments};
use render::pipeline::{DisplayList, DisplayListMode, PipelineCommand, StateOperation};
use render::tui::create_renderer;
use threads::AppThreads;
mod actions;
mod cli;
mod render;
mod threads;

/// This is where the fun starts
fn main() {
    let args = Arguments::parse();
    let state = AppThreads {
        renderer: create_renderer(),
    };

    match args.command {
        ActionCommand::Print(args) => {
            let mut context = ActionContext {
                args: &args,
                pipeline: state.renderer.sink.clone(),
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

    state
        .renderer
        .sink
        .send(DisplayList {
            mode: DisplayListMode::Immediate,
            ops: vec![PipelineCommand::State(StateOperation::Terminate)],
        })
        .expect("Failed to issue pipeline command");

    std::thread::sleep(time::Duration::from_millis(5000));
    println!("Waiting for app threads to clear up and join...");
    state
        .renderer
        .handle
        .join()
        .expect("Something went wrong whilst joining render thread");
}
