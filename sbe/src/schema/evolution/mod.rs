mod compatibility;

use anyhow::Result;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
	/// Check compatibility between two schemas
	Compatibility(compatibility::CompatibilityArgs),
}

pub fn handle(command: Commands) -> Result<()> {
	match command {
		Commands::Compatibility(args) => compatibility::check(args),
	}
}
