mod generate;
mod validate;
mod repo;

use anyhow::Result;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Validate an SBE schema
    Validate(validate::ValidateArgs),
    /// Generate code from an SBE schema
    Generate(generate::GenerateArgs),
    Clone,
    Build,
    Clean,
    CopyJar
}

pub const SBE_VERSION_FILE: &str = "sbe-version.txt";
pub const SBE_JAR_FORMAT: &str = "sbe-all-{version}.jar";

pub fn handle(command: Commands) -> Result<()> {
    match command {
        Commands::Validate(args) => validate::run(args),
        Commands::Generate(args) => generate::run(args),
        Commands::Clone => repo::clone(),
        Commands::Build => repo::build(),
        Commands::Clean => repo::rm_repo_folder(),
        Commands::CopyJar => repo::copy_sbe_jar(),
    }
}