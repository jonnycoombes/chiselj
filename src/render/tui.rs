//! The renderer for text UIs ??

use std::thread;

use super::pipeline::DisplayList;
use crate::render::pipeline::{DisplayListMode, PipelineCommand, StateOperation};
use crate::threads::AppThread;

/// Create the main rendering thread
pub fn create_renderer() -> AppThread<DisplayList<'static>, ()> {
    let (tx, rx) = std::sync::mpsc::channel::<DisplayList>();
    AppThread {
        handle: thread::spawn(move || loop {
            match rx.recv() {
                Ok(list) => {
                    if list.mode == DisplayListMode::Immediate {
                        for op in list.ops {
                            match op {
                                PipelineCommand::State(cmd) => match dbg!(&cmd) {
                                    StateOperation::Terminate => {
                                        println!("Termination requested");
                                        return;
                                    }
                                    _ => (),
                                },
                                PipelineCommand::Render(cmd) => match dbg!(&cmd) {
                                    crate::render::pipeline::DrawOperation::Slice(s) => {
                                        println!("{}", s);
                                    }
                                    _ => (),
                                },
                            }
                        }
                    }
                }
                Err(_) => todo!(),
            }
        }),
        sink: tx,
    }
}
