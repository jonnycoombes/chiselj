use clap::{crate_version, Args, Parser, Subcommand};
use std::path::PathBuf;

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
    /// Print the input
    Print(PrintArgs),
    /// Filter the input
    Filter(FilterArgs),
    /// Manipulate pointers within the input
    Pointers(PointerArgs),
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    /// (Optional) input file.
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct FilterArgs {
    /// (Optional) input file.
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct PointerArgs {
    /// (Optional) input file.
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
}
