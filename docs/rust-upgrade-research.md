# Rust upgrade research

Checked 2026-10-30 against the repository at `ebd043d6fd460f833f64c860662f4e6638c1f078`.

## Verified target

The latest stable release listed by the [official release index](https://blog.rust-lang.org/releases/) is **Rust 1.99.1**, released **2026-10-15**. The [direct announcement](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) confirms the version and date and describes fixes for linking and FFI regressions. Pin `channel = "1.99.1"` for this upgrade rather than selecting a future release or floating channel.

## Repository baseline

Local sources inspected: [toolchain](../rust-toolchain.toml), [workspace manifest](../Cargo.toml), [library manifest](../sbe-schema/Cargo.toml), [CLI manifest](../sbe/Cargo.toml), [CI](../.github/workflows/main.yml), and [format configuration](../.rustfmt.toml).

- Toolchain is pinned to `1.79`, with the default profile.
- Both packages use edition 2021. The root is a virtual workspace with resolver 2. Neither package declares `rust-version`.
- Dependency constraints include `quick-xml = "0.36"`, `thiserror = "1.0"`, `git2 = "0.19"`, and `clap = "4.5"`. `semver` is sourced from a fork's moving `allow_missing_minor_patch` branch.
- CI tests floating stable and nightly on Ubuntu. Local toolchain and CI therefore select different compilers. The PR event list has `repoened` rather than `reopened`.
- Formatting runs on nightly, and the format file contains a section explicitly marked as requiring unstable options.

These are observations of local files, not assertions that the declared minimum dependency versions are the versions currently resolved.

## Recommended migration sequence

1. Establish a build/test baseline and record exact tool versions and resolved dependencies. Determine whether a lockfile is present/tracked; this workspace includes a CLI, so track one for repeatable development and CI.
2. Upgrade the compiler to **1.99.1 while retaining edition 2021 and resolver 2**. Run workspace tests, CLI checks, Clippy, and the chosen formatter. Address compiler/lint failures separately from feature work.
3. Align CI with the pinned compiler. Add a floating stable check if desired; keep nightly as an explicitly separate compatibility or formatting choice. Decide whether to retain nightly formatting or remove unstable format options and use stable formatting.
4. Update dependencies in separate, reviewable changes after the compiler baseline passes. Start with compatible updates, then review breaking upgrades individually. Test XML parsing/serialization across fixtures when changing quick-xml and CLI argument behavior when changing clap. The [quick-xml upstream changelog](https://github.com/tafia/quick-xml/blob/master/Changelog.md) and [clap upstream changelog](https://github.com/clap-rs/clap/blob/main/CHANGELOG.md) are the sources to consult for the exact selected versions; this research does not claim a verified latest version for either crate.
5. Consider edition 2024 as a separate follow-up. The [Edition Guide](https://doc.rust-lang.org/edition-guide/editions/) explains that edition selection is per crate and does not need to change merely to benefit from a newer compiler.

The [Cargo Book](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html) distinguishes dependency requirements from exact locked versions and recommends checking in the lockfile when in doubt. This is especially relevant to the moving Git dependency. The [rustup override documentation](https://rust-lang.github.io/rustup/overrides.html) explains that explicit command-line or environment overrides can take precedence over the checked-in toolchain file.

## Edition 2024 follow-up

Use a clean working tree and run `cargo +1.99.1 fix --edition --workspace --all-targets --all-features` while the packages still declare edition 2021. Review automatic changes, set both package editions to 2024, then rebuild/test and format. The [official migration guide](https://doc.rust-lang.org/stable/edition-guide/editions/transitioning-an-existing-project-to-a-new-edition.html) documents this order. [Cargo's fix documentation](https://doc.rust-lang.org/cargo/commands/cargo-fix.html) notes that inactive feature/platform code may need separate runs.

For this virtual workspace, explicitly change `[workspace].resolver` to `"3"` if adopting the Rust-version-aware resolver. The [2024 resolver guide](https://doc.rust-lang.org/edition-guide/rust-2024/cargo-resolver.html) explains that virtual workspaces do not infer this from member editions. Select and test a deliberate minimum supported Rust version before adding `rust-version`; do not present the development compiler pin as a proven MSRV.

## Limits

This note verifies the current compiler release and migration process. It does not demonstrate successful compilation on Rust 1.99.1, determine an MSRV, audit dependencies, or establish schema compatibility. Those require the repository baseline and behavioral verification in the implementation plan.
