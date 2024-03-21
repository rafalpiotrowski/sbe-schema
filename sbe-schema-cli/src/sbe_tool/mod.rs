mod generate;
mod validate;

use anyhow::Result;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Validate an SBE schema
    Validate(validate::ValidateArgs),
    /// Generate code from an SBE schema
    Generate(generate::GenerateArgs),
}

pub fn handle(command: Commands) -> Result<()> {
    match command {
        Commands::Validate(args) => validate::run(args),
        Commands::Generate(args) => generate::run(args),
    }
}