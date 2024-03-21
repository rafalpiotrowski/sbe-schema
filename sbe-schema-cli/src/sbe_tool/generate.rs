use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::{command, Parser, ValueEnum};
use std::{
    process::Command
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct GenerateArgs {
    #[arg(long, short)]
    pub file: PathBuf,
    #[arg(long, short)]
    pub output_dir: Option<PathBuf>,
    #[arg(long, short)]
    pub package: Option<String>,
    #[arg(long, short)]
    pub namespace: Option<String>,
    #[arg(long, short)]
    pub xinclude: Option<bool>,
    #[arg(long, short, value_enum)]
    pub language: Language,
    #[arg(long, short)]
    pub jar: PathBuf,
    #[arg(long)]
    pub java: Option<PathBuf>
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Language {
    Rust,
    Java,
    Csharp,
    TypeScript,
    Go
}

pub fn run(args: GenerateArgs) -> Result<()> {
    dbg!(&args);

    let output = Command::new(&args.java.unwrap_or("java".into()))
        .arg(format!("-Dsbe.output.dir={:?}", &args.output_dir.unwrap_or(".".into())))
        .arg(format!("-Dsbe.xinclude.aware={:?}", &args.xinclude.unwrap_or(false)))
        .arg(format!("-Dsbe.target.language={:?}", &args.language))
        // .arg("-Dsbe.target.namespace=ace_messages")
        .arg("-jar")
        .arg(format!("{:?}", &args.jar))
        .arg(format!("{:?}", &args.file))
        .output()
        .expect("Unable to execute SBE compiler");

    if !output.status.success() {
        let stderr = std::str::from_utf8(&output.stderr).unwrap();
        panic!("SBE generation failed\n{}", stderr);
    }
    Ok(())
}