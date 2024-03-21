//! Functions relating to printing different levels of info to the terminal.

use std::io::{self, Write};

use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

pub use termcolor::Color;

/// Reset stdout
pub fn reset() -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.reset()?;
    Ok(())
}

/// Write a message
pub fn write(text: &str, color: Color) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::default().set_fg(Some(color)))?;
    writeln!(&mut stdout, "{text}")?;
    Ok(())
}

/// Write a warning message
pub fn warn(text: &str) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))?;
    writeln!(&mut stdout, "{text}")?;
    Ok(())
}

/// Write a debug message
pub fn debug(text: &str) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
    writeln!(&mut stdout, "{text}")?;
    Ok(())
}

/// Write a info message
pub fn info(text: &str) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    writeln!(&mut stdout, "{text}")?;
    Ok(())
}

/// Write a success message
pub fn success(text: &str) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    writeln!(&mut stdout, "{text}")?;
    Ok(())
}

/// Write an error message
pub fn error(text: &str) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
    writeln!(&mut stdout, "{text}")?;
    Ok(())
}