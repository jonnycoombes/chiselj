//! The renderer for text UIs
use super::commands::{CommandList, PipelineCommand, RenderCommand, StateCommand};
use super::{options::RenderOptions, themes::Theme};
use crate::threads::AppThread;
use crossterm::terminal;
use std::io::{stdout, Write};
use std::sync::mpsc::{channel, Receiver};
use std::thread;

#[derive(Debug, Copy, Clone, PartialEq)]
enum RendererControlCode {
    Continue,
    Terminate,
}

/// Used to track the internal render state
#[derive(Debug, Copy, Clone)]
struct RenderState {
    /// The initially configured [RenderOptions]
    pub options: RenderOptions,
    /// The current control code (governs the overall state of the rendering loop)
    pub control_code: RendererControlCode,
    /// The current theme information
    pub theme: Theme,
}

/// Create a new rendering thread
pub fn new_renderer(options: RenderOptions) -> AppThread<CommandList, ()> {
    let (tx, rx) = channel::<CommandList>();
    AppThread {
        handle: Some(thread::spawn(move || render(options, rx))),
        sink: tx,
    }
}

/// Construct the initial rendering state
fn initial_render_state(options: &RenderOptions) -> RenderState {
    RenderState {
        options: *options,
        control_code: RendererControlCode::Continue,
        theme: Theme { indent: ' ' },
    }
}

/// The main rendering logic flows out from here
#[cfg(feature = "crossterm")]
fn render(options: RenderOptions, pipeline: Receiver<CommandList>) {
    use super::commands::CommandListMode;

    let mut state = initial_render_state(&options);
    if state.options.raw {
        terminal::enable_raw_mode().unwrap();
    }

    // default to stdout but this may become pluggable in the future
    let mut stdout = stdout();
    loop {
        match pipeline.recv() {
            Ok(list) => {
                if list.mode == CommandListMode::Immediate {
                    for cmd in list.cmds {
                        state = match cmd {
                            PipelineCommand::State(inner) => {
                                handle_state_command(&mut state, &inner)
                            }
                            PipelineCommand::Render(inner) => {
                                handle_render_command(&mut stdout, &mut state, &inner)
                            }
                        };
                        if state.control_code == RendererControlCode::Terminate {
                            terminal::disable_raw_mode().unwrap();
                            return;
                        }
                    }
                }
            }
            Err(_) => (),
        }
    }
}

/// Update the current rendering state with information relating to cursor position, terminal size
/// etc...
#[inline]
fn update_render_state(state: &mut RenderState) -> RenderState {
    *state
}

/// Handle any [PipelineCommand::State] commands
#[cfg(feature = "crossterm")]
fn handle_state_command(state: &mut RenderState, cmd: &StateCommand) -> RenderState {
    match cmd {
        StateCommand::Terminate => state.control_code = RendererControlCode::Terminate,
        _ => (),
    }
    update_render_state(state)
}

/// Handle any [PipelineCommand::RenderCommand] commands
#[cfg(feature = "crossterm")]
fn handle_render_command(
    out: &mut dyn Write,
    state: &mut RenderState,
    cmd: &RenderCommand,
) -> RenderState {
    let _result = match cmd {
        RenderCommand::NewLine => write!(out, "\n"),
        RenderCommand::Indent(n) => {
            for _ in 0..*n {
                let _ = write!(out, "{}", state.theme.indent);
            }
            Ok(())
        }
        RenderCommand::Char(c) => write!(out, "{}", c),
        RenderCommand::Repeat(c, n) => {
            for _ in 0..*n {
                let _ = write!(out, "{}", c);
            }
            Ok(())
        }
        RenderCommand::FixedWidthText(_, _) => todo!(),
        RenderCommand::Text(s) => write!(out, "{}", s),
        RenderCommand::Slice(s) => write!(out, "{}", s),
    };
    update_render_state(state)
}
