mod repo;

use anyhow::Result;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Clone,
    Build,
    Clean,
    CopyJar,
}

pub const SBE_VERSION_FILE: &str = "sbe-version.txt";
pub const SBE_JAR_FORMAT: &str = "sbe-all-{version}.jar";

pub fn handle(command: Commands) -> Result<()> {
    match command {
        Commands::Clone => repo::clone(),
        Commands::Build => repo::build(),
        Commands::Clean => repo::clean(),
        Commands::CopyJar => repo::copy_sbe_jar(),
    }
}
