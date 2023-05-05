//! All rendering to the TUI is carried out through a pipeline and display list abstraction so that sequences of
//! rendering operations can be sent to a specific renderer grouped together into batches
//!

#[derive(Debug, Clone, PartialEq)]
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

/// Primitive drawing operations
#[derive(Debug, Clone, PartialEq)]
pub enum DrawOperation<'a> {
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
    Slice(&'a str),
}

/// Render state mutation operations
#[derive(Debug, PartialEq)]
pub enum StateOperation {
    /// Clear the current buffer
    Clear,
    /// Set the render cursor position
    SetCursor(u16, u16),
    /// Push a colour
    PushColour(u8, u8, u8),
    /// Pop a colour
    PopColour,
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
pub enum PipelineCommand<'a> {
    /// A command to mutate the state of the pipeline
    State(StateOperation),
    /// Render, using a specific op code
    Render(DrawOperation<'a>),
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

/// A display list is currently just a vector of [RenderCommand]s
pub struct DisplayList<'a> {
    /// The mode for the display list
    pub mode: DisplayListMode,
    /// The operations associated with the display list
    pub ops: Vec<PipelineCommand<'a>>,
}

/// A pipeline is a wrapper around a channel and some other stuffs
pub type Pipeline<'a> = std::sync::mpsc::Sender<DisplayList<'a>>;
