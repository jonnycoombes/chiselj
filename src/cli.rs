use clap::{crate_version, Command, Parser};

/// Top level command line arguments and configuration settings
#[derive(Parser)]
#[command(name = "chisel")]
#[command(author = "Jonny Coombes <jcoombes@jcs-software.co.uk>")]
#[command(about = "A simple command line JSON wrangler", long_about = None)]
#[command(version = crate_version!())]
pub struct Arguments {}
