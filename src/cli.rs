use clap::{crate_version, Args, Parser, Subcommand};
use std::{ffi::OsString, path::PathBuf};

/// Top level command line arguments and configuration settings
#[derive(Parser)]
#[command(name = "chisel")]
#[command(author = "Jonny Coombes <jcoombes@jcs-software.co.uk>")]
#[command(about = "A simple command line JSON wrangler", long_about = None)]
#[command(version = crate_version!())]
pub struct Arguments {
    #[command(subcommand)]
    pub command: ActionCommand,
}

/// Enumeration of available commands and their associated arguments
#[derive(Debug, Subcommand)]
pub enum ActionCommand {
    /// Pretty printing
    Print(PrintArgs),
    /// Filtering
    Filter(FilterArgs),
    /// Inspecting and manipulating JSON pointers
    Pointers(PointerArgs),
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    /// Source JSON file. If not specified, input is assumed to come from stdin.
    #[arg(last = true, value_name = "FILE")]
    pub file: Option<OsString>,

    /// Indent space count
    ///
    /// Object keys and array values are idented by this amount plus the parent identation amount
    #[arg(short, long, value_name = "indent", default_value = "2")]
    pub indent: u16,

    /// KV padding count
    ///
    /// The number of spaces added to each side of the ":" character in a <key> : <value> pair
    #[arg(short, long, value_name = "kvpadding", default_value = "1")]
    pub kvpadding: u16,
}

#[derive(Debug, Args)]
pub struct FilterArgs {
    /// (Optional) input file.
    #[arg(short, long, value_name = "FILE")]
    pub file: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct PointerArgs {
    /// (Optional) input file.
    #[arg(short, long, value_name = "FILE")]
    pub file: Option<PathBuf>,
}
