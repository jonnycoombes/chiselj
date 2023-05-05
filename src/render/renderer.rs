//! The renderer for text UIs ??
use super::operations::{DisplayList, DrawOperation, PipelineCommand, StateOperation};
use crate::threads::AppThread;
use std::sync::mpsc::{channel, Receiver};
use std::thread;

struct RenderState {}

/// Create a new rendering thread
pub fn new_renderer() -> AppThread<DisplayList, ()> {
    let (tx, rx) = channel::<DisplayList>();
    AppThread {
        handle: Some(thread::spawn(move || render(rx))),
        sink: tx,
    }
}

/// The main rendering logic flows out from here
#[cfg(feature = "crossterm")]
fn render(source: Receiver<DisplayList>) {
    let _state = RenderState {};
    loop {
        match source.recv() {
            Ok(list) => {
                for op in list.ops {
                    match op {
                        PipelineCommand::State(cmd) => match &cmd {
                            StateOperation::Terminate => {
                                println!("Termination requested");
                                return;
                            }
                            _ => (),
                        },
                        PipelineCommand::Render(cmd) => match &cmd {
                            DrawOperation::Slice(s) => {
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

/// Handle any [PipelineCommand::State] operations
#[cfg(feature = "crossterm")]
fn handle_state_operation(_state: &mut RenderState, _op: StateOperation) {}

/// Handle any [PipelineCommand::DrawOperation] operations
#[cfg(feature = "crossterm")]
fn handle_draw_operation(_state: &mut RenderState, _op: DrawOperation) {}
