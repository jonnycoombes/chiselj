//! The renderer for text UIs
use super::display_lists::{ChangeState, DisplayList, DisplayListCommand, DisplayListMode, Draw};
use super::{options::DrawOptions, themes::Theme};
use crate::threads::AppThread;
use crossterm::terminal;
use std::io::{stdout, Write};
use std::sync::mpsc::{channel, Receiver};
use std::thread;

#[derive(Debug, Copy, Clone, PartialEq)]
enum DrawerControlCode {
    Continue,
    Terminate,
}

/// Used to track the internal render state
#[derive(Debug, Copy, Clone)]
struct DrawChangeState {
    /// The initially configured [DrawOptions]
    pub options: DrawOptions,
    /// The current control code (governs the overall state of the rendering loop)
    pub control_code: DrawerControlCode,
    /// The current theme information
    pub theme: Theme,
}

/// Create a new rendering thread
pub fn new_renderer(options: DrawOptions) -> AppThread<DisplayList, ()> {
    let (tx, rx) = channel::<DisplayList>();
    AppThread {
        handle: Some(thread::spawn(move || render(options, rx))),
        sink: tx,
    }
}

/// Construct the initial rendering state
fn initial_render_state(options: &DrawOptions) -> DrawChangeState {
    DrawChangeState {
        options: *options,
        control_code: DrawerControlCode::Continue,
        theme: Theme { indent: ' ' },
    }
}

/// The main rendering logic flows out from here
#[cfg(feature = "crossterm")]
fn render(options: DrawOptions, pipeline: Receiver<DisplayList>) {
    let mut state = initial_render_state(&options);
    if state.options.raw {
        terminal::enable_raw_mode().unwrap();
    }

    // default to stdout but this may become pluggable in the future
    let mut stdout = stdout();
    loop {
        match pipeline.recv() {
            Ok(list) => {
                if list.mode == DisplayListMode::Immediate {
                    for cmd in list.cmds {
                        state = match cmd {
                            DisplayListCommand::ChangeState(inner) => {
                                handle_state_command(&mut state, &inner)
                            }
                            DisplayListCommand::Draw(inner) => {
                                handle_render_command(&mut stdout, &mut state, &inner)
                            }
                        };
                        if state.control_code == DrawerControlCode::Terminate {
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
fn update_render_state(state: &mut DrawChangeState) -> DrawChangeState {
    *state
}

/// Handle any [DisplayListCommand::ChangeState] commands
#[cfg(feature = "crossterm")]
fn handle_state_command(state: &mut DrawChangeState, cmd: &ChangeState) -> DrawChangeState {
    match cmd {
        ChangeState::Terminate => state.control_code = DrawerControlCode::Terminate,
        _ => (),
    }
    update_render_state(state)
}

/// Handle any [DisplayListCommand::Draw] commands
#[cfg(feature = "crossterm")]
fn handle_render_command(
    out: &mut dyn Write,
    state: &mut DrawChangeState,
    cmd: &Draw,
) -> DrawChangeState {
    let _result = match cmd {
        Draw::NewLine => write!(out, "\n"),
        Draw::Indent(n) => {
            for _ in 0..*n {
                let _ = write!(out, "{}", state.theme.indent);
            }
            Ok(())
        }
        Draw::Char(c) => write!(out, "{}", c),
        Draw::Repeat(c, n) => {
            for _ in 0..*n {
                let _ = write!(out, "{}", c);
            }
            Ok(())
        }
        Draw::FixedWidthText(_, _) => todo!(),
        Draw::Text(s) => write!(out, "{}", s),
        Draw::Slice(s) => write!(out, "{}", s),
    };
    update_render_state(state)
}
