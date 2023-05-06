//! The renderer for text UIs ??
use super::commands::{CommandList, PipelineCommand, RenderCommand, StateCommand};
use super::themes::Theme;
use crate::threads::AppThread;
use crossterm::{cursor, terminal};
use std::sync::mpsc::{channel, Receiver};
use std::thread;

/// Structure used for setting initial rendering options
#[derive(Debug, Clone, Copy)]
pub struct RenderOptions {
    /// Should raw mode be enabled?
    pub raw: bool,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum RenderControlCode {
    Continue,
    Terminate,
}

/// Used to track the internal render state
#[derive(Debug, Copy, Clone)]
struct RenderState {
    /// The initially configured [RenderOptions]
    pub options: RenderOptions,
    /// The current terminal size
    pub size: (u16, u16),
    /// The current cursor position
    pub position: (u16, u16),
    /// The current control code (governs the overall state of the rendering loop)
    pub control_code: RenderControlCode,
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
        size: terminal::size().unwrap(),
        position: cursor::position().unwrap(),
        control_code: RenderControlCode::Continue,
        theme: Theme { space_char: '.' },
    }
}

/// The main rendering logic flows out from here
#[cfg(feature = "crossterm")]
fn render(options: RenderOptions, pipeline: Receiver<CommandList>) {
    let mut state = initial_render_state(&options);
    loop {
        match pipeline.recv() {
            Ok(list) => {
                for cmd in list.cmds {
                    state = match cmd {
                        PipelineCommand::State(inner) => handle_state_command(&mut state, &inner),
                        PipelineCommand::Render(inner) => handle_render_command(&mut state, &inner),
                    };
                    if state.control_code == RenderControlCode::Terminate {
                        return;
                    }
                }
            }
            Err(_) => todo!(),
        }
    }
}

/// Update the current rendering state with information relating to cursor position, terminal size
/// etc...
#[inline]
fn update_render_state(state: &mut RenderState) -> RenderState {
    state.position = cursor::position().unwrap();
    state.size = terminal::size().unwrap();
    *state
}

/// Handle any [PipelineCommand::State] commands
#[cfg(feature = "crossterm")]
fn handle_state_command(state: &mut RenderState, cmd: &StateCommand) -> RenderState {
    match cmd {
        StateCommand::Terminate => state.control_code = RenderControlCode::Terminate,
        _ => (),
    }
    update_render_state(state)
}

/// Handle any [PipelineCommand::RenderCommand] commands
#[cfg(feature = "crossterm")]
fn handle_render_command(state: &mut RenderState, cmd: &RenderCommand) -> RenderState {
    match cmd {
        RenderCommand::NewLine => println!(""),
        RenderCommand::Indent(n) => {
            for _ in 0..*n {
                print!("{}", state.theme.space_char)
            }
        }
        RenderCommand::Char(c) => print!("{}", c),
        RenderCommand::Repeat(_, _) => todo!(),
        RenderCommand::FixedWidthText(_, _) => todo!(),
        RenderCommand::Text(s) => print!("{}", s),
        RenderCommand::Slice(s) => print!("{}", s),
    }
    update_render_state(state)
}
