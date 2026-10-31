# Rust upgrade and functional rollback status

Updated 2026-10-31. The functional redesign has been rolled back to the implementation at `ebd043d6`. Both packages remain version **0.1.0**. This change prepares the compatible Rust upgrade for review; it does not publish either crate.

## Restored

- Original typed schema model, direct field editing, `Default`, Serde input/output and XML serialization.
- Original strategy/validator traits and wrappers, Optional/PartialCompatibility, compatibility variants, VTable APIs and result/error types.
- Original C# routing and language options, Java/custom JAR paths, independent package/namespace options and short flags.
- Original executable name `sbe-cli`, command names and evolution arguments.
- Original `.simple-binary-encoding` clone/build workflow, current-directory JAR/version file discovery, copy/cleanup behavior, and support for non-pinned or modified source checkouts.
- Embedded git2, terminal colors and progress output.
- Original dependency requirements and semver fork, including abbreviated `5.2` parsing. The lockfile fixes the fork revision to `7d68884b41cea9fd98601365902b1afac7b631eb`.

The replacement ordered-document model, new native validation/compatibility engine, restricted backend list, pinned-only source/cache implementation, JSON CLI reports and 0.2 API/version changes were removed. Their proposed improvements are recorded as future work rather than shipped as replacements.

## Retained compatible work

- Rust 1.99.1 development toolchain, edition 2021, stable rustfmt and Clippy. No new package MSRV declaration.
- Tracked lockfile and updated CI using direct Cargo commands, reviewed action revisions, Linux/Windows and pinned/floating stable compilers.
- Formatting, explicit/elided equivalent lifetimes, unused-import/redundant-borrow cleanup, corrected doc link and comments. Existing control flow and decision logic are retained.
- A Windows-only build script links `advapi32` required by the original libgit2 dependency. Building the actual CLI exposed this dependency linkage issue; replacing git2 was not necessary.
- Both declared license texts included in the crate directories.
- Six new contract tests cover Serde XML read/edit/write, original API/wrapper use, abbreviated semantic versions, language/flag exposure and original commands. The two original tests remain.

## Verification

Eight tests pass on Rust 1.99.1: two original tests and six contract tests. Clippy passes with warnings denied; formatting, documentation tests and API documentation checks pass. Linux CI is configured but has not run in this session.

The rollback was checked against the original production-source diff at `ebd043d6`.
Remaining source differences are formatting, equivalent lifetime annotations,
lint/documentation cleanup and the Windows linkage fix. No tracked files remain
deleted. The original dependencies and package versions are restored.

This rollback deliberately restores the previous behavior of unfinished validation and evolution commands as well. It does not claim that those original implementations are complete or correct. The [implementation plan](implementation-plan.md) records how to improve them without losing existing capabilities.
