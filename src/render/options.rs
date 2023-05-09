/// Structure used for setting initial rendering options
#[derive(Debug, Clone, Copy)]
pub struct RenderOptions {
    /// Should raw mode be enabled?
    pub raw: bool,
}

/// Setup a sensible set of defaults for the rendering options
impl Default for RenderOptions {
    fn default() -> Self {
        Self { raw: false }
    }
}
