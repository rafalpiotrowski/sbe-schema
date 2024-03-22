use std::path::{Path, PathBuf};

use anyhow::{bail, Result};
use clap::{command, Parser, ValueEnum};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct GenerateArgs {
    /// schema file to generate code from
    #[arg(long, short)]
    pub file: PathBuf,
    /// output directory for generated code, default "generated"
    #[arg(long, short)]
    pub output_dir: Option<PathBuf>,
    /// package name for generated code
    #[arg(long, short)]
    pub package: Option<String>,
    /// namespace for generated code
    #[arg(long, short)]
    pub namespace: Option<String>,
    /// enable xinclude processing, default true
    #[arg(long, short)]
    pub xinclude: Option<bool>,
    /// target language for generated code
    #[arg(long, short, value_enum)]
    pub language: Language,
    /// path to the SBE jar file, default uses the version in the SBE_VERSION file and current working directory
    #[arg(long, short)]
    pub jar: Option<PathBuf>,
    /// path to the Java executable, default "java" and uses PATH
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

    let mut cmd = Command::new(&args.java.unwrap_or("java".into()));

    cmd.arg(format!("-Dsbe.output.dir={0}", &args.output_dir.unwrap_or("generated".into()).to_str().unwrap()))
        .arg(format!("-Dsbe.xinclude.aware={:?}", &args.xinclude.unwrap_or(true)));

    match &args.language {
        Language::Csharp => {
            cmd.arg("-Dsbe.target.language=uk.co.real_logic.sbe.generation.csharp.CSharp");
        },
        Language::TypeScript => {
            panic!("TypeScript is not supported yet")
        },
        _ => {
            cmd.arg(format!("-Dsbe.target.language={:?}", &args.language));
        }
    }

    if let Some(package) = &args.package {
        cmd.arg(format!("-Dsbe.target.package={:?}", package));
    }

    if let Some(namespace) = &args.namespace {
        cmd.arg(format!("-Dsbe.target.namespace={:?}", namespace));
    }

    if let Some(jar) = &args.jar {    
        cmd.arg("-jar").arg(jar);
    } else {
        let version_file = Path::new(crate::tool::SBE_VERSION_FILE);
        let version = std::fs::read_to_string(version_file)?;
        let jar_file = crate::tool::SBE_JAR_FORMAT.replace("{version}", &version.trim());
        cmd.arg("-jar").arg(jar_file);
    }

    cmd.arg(&args.file);

    dbg!(&cmd);

    let output = cmd.output()
        .expect("Unable to execute SBE compiler");

    if !output.status.success() {
        let stderr = std::str::from_utf8(&output.stderr).unwrap();
        bail!("SBE generation failed\n{}", stderr);
    }
    Ok(())
}