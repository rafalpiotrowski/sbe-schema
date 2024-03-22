use anyhow::Result;
use clap::{command, Parser, ValueEnum};
use sbe_schema::NoneStrategy;

use crate::term::info;

///
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CompatibilityArgs {
    #[arg(long, short)]
    pub level: CompatibilityLevel,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum CompatibilityLevel {
    Backward,
    Forward,
    Full,
    None,
}

pub fn check(args: CompatibilityArgs) -> Result<()> {

    match args {
        CompatibilityArgs { level: CompatibilityLevel::Backward } => {
            info("Checking backward compatibility")?;
        },
        CompatibilityArgs { level: CompatibilityLevel::Forward } => {
            info("Checking forward compatibility")?;
        },
        CompatibilityArgs { level: CompatibilityLevel::Full } => {
            info("Checking full compatibility")?;
        },
        CompatibilityArgs { level: CompatibilityLevel::None } => {
            info("Checking no compatibility")?;
            let v = sbe_schema::Validator::new(NoneStrategy);
            v.check(sbe_schema::Schema::default(), sbe_schema::Schema::default())?;
        },
    }

    Ok(())
}
