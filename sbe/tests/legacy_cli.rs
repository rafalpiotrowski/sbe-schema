//! Regression coverage for the original CLI command and generator options.
#![allow(unused_crate_dependencies)]

use std::process::Command;

fn help(arguments: &[&str]) -> String {
	let output = Command::new(env!("CARGO_BIN_EXE_sbe-cli")).args(arguments).output().unwrap();
	assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
	String::from_utf8(output.stdout).unwrap()
}

#[test]
fn original_generator_languages_and_independent_options_remain_exposed() {
	let output = help(&["schema", "generate", "--help"]);
	for option in [
		"--package",
		"--namespace",
		"--java",
		"--jar",
		"--xinclude",
		"csharp",
		"type-script",
		"rust",
		"java",
		"go",
	] {
		assert!(output.contains(option), "missing {option}: {output}");
	}
	assert!(output.contains("-p,"));
	assert!(output.contains("-n,"));
}

#[test]
fn original_tool_and_evolution_commands_remain_available() {
	let output = help(&["tool", "--help"]);
	for command in ["clone", "build", "copy-jar", "clean"] {
		assert!(output.contains(command));
	}
	let output = help(&["schema", "evolution", "compatibility", "--help"]);
	for option in ["--latest", "--current", "--level", "backward", "forward", "full", "none"] {
		assert!(output.contains(option), "missing {option}: {output}");
	}
}
