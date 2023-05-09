use std::path::PathBuf;

use super::Command;
use clap::Args;

///
#[derive(Debug, Args)]
pub struct FilterCommand {
    /// Source JSON file. If not specified, input is assumed to come from stdin.
    #[arg(last = true, value_name = "FILE")]
    pub file: Option<PathBuf>,
}

impl Command for FilterCommand {
    fn execute(&self, _context: &mut super::CommandContext) -> crate::errors::ChiselResult<()> {
        todo!()
    }
}
