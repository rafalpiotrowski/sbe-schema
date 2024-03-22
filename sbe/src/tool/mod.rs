mod repo;

use anyhow::Result;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// clone SBE github repo to current directory under .simple-binary-encoding
    Clone,
    /// build SBE using gradlew in the .simple-binary-encoding directory
    Build,
    /// removed .simple-binary-encoding directory and SBE jar
    Clean,
    /// copy the SBE jar to the current directory, build will do it but we can do it manually as well
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
