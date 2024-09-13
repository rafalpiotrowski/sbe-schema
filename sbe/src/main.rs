//! # sbe
//!
//! `sbe` is a command line tool for working with SBE schema files.
//!
//! ## Usage
//!
//! ```shell
//! sbe --help
//! ```
//!
//! ## Example
//!
//! ```shell
//! sbe schema generate -l rust -f schema.xml
//! ```
mod schema;
mod term;
mod tool;

use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about = "SBE schema tool", long_about = None, propagate_version = true)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	/// Work with SBE schema files: validate and generate code for different languages
	#[command(subcommand)]
	Schema(schema::Commands),
	/// Work with SBE source code. Clone, build, and copy jar file for later use in code generation
	/// and schema validation. Requires to have java installed and available in the PATH or
	/// specify the path to the java executable.
	#[command(subcommand)]
	Tool(tool::Commands),
}

fn main() {
	let cli = Cli::parse();

	let result = match cli.command {
		Commands::Schema(args) => schema::handle(args),
		Commands::Tool(args) => tool::handle(args),
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
