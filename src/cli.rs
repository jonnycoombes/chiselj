use clap::{crate_version, Command, Parser};

#[derive(Parser)]
#[command(name = "chisel")]
#[command(author = "Jonny Coombes <jcoombes@jcs-software.co.uk>")]
#[command(about = "JSON wrangler", long_about = None)]
#[command(version = crate_version!())]
pub struct Cli {}
