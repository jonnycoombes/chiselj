//! The renderer for text UIs ??
use super::commands::{DisplayList, PipelineCommand, RenderCommand, StateCommand};
use crate::threads::AppThread;
use std::sync::mpsc::{channel, Receiver};
use std::thread;

use crossterm::{cursor, terminal};

/// Structure used for setting initial rendering options
#[derive(Debug, Clone, Copy)]
pub struct RenderOptions {
    /// Should raw mode be enabled?
    pub raw: bool,
}

/// Used to track the internal render state
#[derive(Debug)]
struct RenderState {
    /// The initially configured [RenderOptions]
    pub options: RenderOptions,
    /// The current terminal size
    pub size: (u16, u16),
    /// The current cursor position
    pub position: (u16, u16),
}

/// Create a new rendering thread
pub fn new_renderer(options: RenderOptions) -> AppThread<DisplayList, ()> {
    let (tx, rx) = channel::<DisplayList>();
    AppThread {
        handle: Some(thread::spawn(move || render(options, rx))),
        sink: tx,
    }
}

/// The main rendering logic flows out from here
#[cfg(feature = "crossterm")]
fn render(options: RenderOptions, pipeline: Receiver<DisplayList>) {
    let _state = RenderState {
        options,
        size: terminal::size().unwrap(),
        position: cursor::position().unwrap(),
    };
    loop {
        match pipeline.recv() {
            Ok(list) => {
                for cmd in list.cmds {
                    match cmd {
                        PipelineCommand::State(inner) => match &inner {
                            StateCommand::Terminate => {
                                println!("Termination requested");
                                return;
                            }
                            _ => (),
                        },
                        PipelineCommand::Render(inner) => match &inner {
                            RenderCommand::Slice(s) => {
                                println!("{}", s);
                            }
                            _ => (),
                        },
                    }
                }
            }
            Err(_) => todo!(),
        }
    }
}

/// Handle any [PipelineCommand::State] commands
#[cfg(feature = "crossterm")]
fn handle_state_command(_state: &mut RenderState, _op: StateCommand) {}

/// Handle any [PipelineCommand::RenderCommand] commands
#[cfg(feature = "crossterm")]
fn handle_draw_command(_state: &mut RenderState, _op: RenderCommand) {}
