//! Supply the Windows system library required by the existing libgit2 dependency.

fn main() {
	if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
		println!("cargo:rustc-link-lib=advapi32");
	}
}
