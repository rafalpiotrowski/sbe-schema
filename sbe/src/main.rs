//! # sbe-schema-cli
//!
//! `sbe-schema-cli` is a command line tool for working with SBE schema files.
//!
//! ## Usage
//!
//! ```shell
//! sbe-schema-cli --help
//! ```
//!
//! ## Example
//!
//! ```shell
//! sbe-schema-cli evolution compatibility --level full
//! ```
//!
mod evolution;
mod sbe_tool;
mod term;

use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about = "SBE schema tool", long_about = None, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// check schema evolution compatibility
    #[command(subcommand)]
    Evolution(evolution::Commands),
    /// work with SBE schemas using sbe-tool
    #[command(subcommand)]
    SbeTool(sbe_tool::Commands),
}

fn main() {
    let cli = Cli::parse();
    
    let result = match cli.command {
        Commands::Evolution(args) => evolution::handle(args),
        Commands::SbeTool(args) => sbe_tool::handle(args),
    };
    
    if let Err(e) = &result {
        if term::error(&format!("{e}")).is_err() {
            // if we can't color the error message, just eprint it
            eprintln!("{e}");
        }
    }

    // reset terminal colors
    _ = term::reset();

    match result {
        Ok(_) => std::process::exit(exitcode::OK),
        Err(_) => std::process::exit(exitcode::DATAERR),
    }
}
