use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ValidateArgs {
    /// 
    #[arg(long, short)]
    pub file: PathBuf,
}

pub fn run(args: ValidateArgs) -> Result<()> {
    Ok(())
}