use clap::{crate_version, Parser, Subcommand};

use crate::commands::filter::FilterCommand;
use crate::commands::pointers::PointersCommand;
use crate::commands::print::PrintCommand;

/// Top level command line arguments and configuration settings
#[derive(Parser)]
#[command(name = "chisel")]
#[command(author = "Jonny Coombes <jcoombes@jcs-software.co.uk>")]
#[command(about = "A simple command line JSON wrangler", long_about = None)]
#[command(version = crate_version!())]
pub struct AppArguments {
    #[command(subcommand)]
    pub command: AppCommand,
}

/// Enumeration of available commands and their associated arguments
#[derive(Debug, Subcommand)]
pub enum AppCommand {
    #[command(about = "Pretty printing JSON DOM structures", long_about = None)]
    Print(PrintCommand),
    #[command(about = "Filtering JSON DOM structures", long_about = None)]
    Filter(FilterCommand),
    #[command(about = "Inspecting JSON pointers", long_about = None)]
    Pointers(PointersCommand),
}
