use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct CliArgs {
    #[arg(short, long)]
    pub file: Option<PathBuf>,
    /// Either pass the connection string here or through CONNECTION_STRING in .env
    #[arg(short, long)]
    pub connection: Option<String>,
}
