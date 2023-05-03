/// Allowable alignment values
#[derive(Debug, Clone)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

/// Primitive rendering operations used for output and state management
#[derive(Debug, Clone)]
pub enum OpCode {
    /// Output a newline
    NewLine,
    /// Output a single character
    Char(char),
    /// Output a repeated character count times
    Repeat(char, usize),
    /// Push an alignment
    PushAlignment(Alignment),
    /// Pop an alignment
    PopAlignment,
    /// Push a colour
    PushColour(u8, u8, u8),
    /// Pop a colour
    PopColour,
    /// Push an indent
    PushIndent(u8),
    /// Pop an indent
    PopIndent,
    /// Push a bold style
    PushBold,
    /// Pop a bold style
    PopBold,
    /// Push an italic style
    PushItalic,
    /// Pop an italic style
    PopItalic,
    /// Output fixed width text
    FixedWidthText(String, usize),
    /// Output text
    Text(String),
    /// Output a static slice
    Slice(&'static str)

