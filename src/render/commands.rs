#![allow(unused_macros)]
//! All rendering to the TUI is carried out through a pipeline and display list abstraction so that sequences of
//! rendering commands can be sent to a specific renderer grouped together into batches
//!
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontStyle {
    /// Normal font
    Normal,
    /// Bold font
    Bold,
    /// Italic font
    Italic,
}

/// Allowable alignment values
#[derive(Debug, Clone, PartialEq)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

/// Primitive drawing coammands
#[derive(Debug, Clone, PartialEq)]
pub enum Draw {
    /// Output a newline
    NewLine,
    /// Indent by a positive number of chars
    Indent(u16),
    /// Output a single character
    Char(char),
    /// Output a repeated character count times
    Repeat(char, u16),
    /// Output fixed width text
    FixedWidthText(String, u16),
    /// Output text
    Text(String),
    /// Output a static slice
    Slice(&'static str),
}

/// Draw state mutation commands
#[derive(Debug, PartialEq)]
pub enum ChangeState {
    /// Clear the current buffer
    Clear,
    /// Set the render cursor position
    SetCursor(u16, u16),
    /// Push a foreground colour
    PushForegroundColour(u8, u8, u8),
    /// Pop a foreground colour
    PopForegroundColour,
    /// Push a background colour
    PushBackgroundColour(u8, u8, u8),
    /// Pop a background colour
    PopBackgroundColour,
    /// Push a font style
    PushFontStyle(FontStyle),
    /// Pop the current font style
    PopFontStyle,
    /// Pop an alignment
    PopAlignment,
    /// Push an alignment
    PushAlignment(Alignment),
    /// Terminate the
    Terminate,
}

/// A pipeline command is basically just a sum type, either a state related command, or a render
/// command
#[derive(Debug)]
pub enum DisplayListCommand {
    /// A command to mutate render state
    ChangeState(ChangeState),
    /// Draw, using a specific op code
    Draw(Draw),
}

/// Shorthand for creating state operations
#[macro_export]
macro_rules! state {
    ($op : expr) => {
        DisplayListCommand::ChangeState($op)
    };
}

/// Shorthand for creating render operations
#[macro_export]
macro_rules! render {
    ($op : expr) => {
        DisplayListCommand::Draw($op)
    };
}

/// A display list can either be immediate (meaning render immediately) or deferred (meaning that
/// the renderer may decide to not render immediately)
#[derive(Debug, PartialEq)]
pub enum DisplayListMode {
    /// The display list should be rendered immediately
    Immediate,
    /// The display list may be deferred and rendered later
    Deferred,
}

/// A display list is currently just a vector of [Draw]s
#[derive(Debug)]
pub struct DisplayList {
    /// The mode for the display list
    pub mode: DisplayListMode,
    /// The operations associated with the display list
    pub cmds: Vec<DisplayListCommand>,
}

/// Helper macro for generating an immediate mode [CommandList]
#[macro_export]
macro_rules! cl_immediate {
    ($($c : expr),+) => {
        DisplayList {
            mode: DisplayListMode::Immediate,
            cmds: vec![$(DisplayListCommand::Draw($c)),*],
        }
    };
    ($($c : expr,)+) => {
        DisplayList {
            mode: DisplayListMode::Immediate,
            cmds: vec![$(DisplayListCommand::Draw($c)),*],
        }
    };
}

/// Helper macro for generating a deferred mode [CommandList]
#[macro_export]
macro_rules! cl_deferred {
    ($($c : expr),+) => {
        DisplayList {
            mode: DisplayListMode::Deferred,
            cmds: vec![$(DisplayListCommand::Draw($c)),*],
        }
    };
    ($($c : expr,)+) => {
        DisplayList {
            mode: CommandListMode::Deferred,
            cmds: vec![$(DisplayListCommand::Draw($c)),*],
        }
    };
}
