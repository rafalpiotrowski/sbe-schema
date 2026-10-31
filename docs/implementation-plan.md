# Implementation plan with functionality preservation

Updated 2026-10-31. Baseline: `ebd043d6`. This supersedes the initial redesign proposal.

The tool manages SBE schemas: a reusable Rust model, schema validation/evolution commands, code generation through the upstream Java compiler, and upstream compiler clone/build/JAR management. Existing library capabilities and CLI workflows are compatibility requirements.

## Current work

Keep the compiler upgrade to Rust 1.99.1 with edition 2021, a tracked lockfile, stable formatting, and CI for Linux/Windows. Preserve the original package versions, dependency requirements and semver fork. Limit production edits to compiler/linter compatibility, documentation, formatting and the Windows system-library linkage needed by the existing git2 dependency.

Add regression coverage for original public APIs, typed schema mutation, Serde XML read/write, short semantic versions, generator language/flag exposure, and legacy tool/evolution commands.

## Compatibility requirements for future work

- Preserve the typed `Schema` and nested model, public fields, `Default`, Serde serialization/deserialization, and XML output.
- Preserve existing strategy/validator/optional/partial-compatibility interfaces, VTable interfaces, result/error names and compatibility variants. Add public exports or adapters where needed rather than removing interfaces.
- Preserve C# generator routing, custom JAR and Java paths, independent package/namespace parameters and short flags. TypeScript remains an unfinished exposed option; replacing its panic with a useful error is a separate behavior change to review.
- Preserve the existing executable name and command names. Add aliases rather than silently renaming them.
- Preserve current-directory `sbe-version.txt` discovery, `.simple-binary-encoding`, JAR copy/cleanup, and source build workflows. Support arbitrary or locally modified upstream checkouts. A pinned artifact/cache can be an opt-in addition.
- Preserve embedded git2 cloning, colored messages and progress reporting. Avoid introducing an external Git requirement for the CLI.
- Keep new validation rules and compatibility policies separate from existing workflows until their behavior and migration are explicitly reviewed. Do not restrict previously supported inputs through a hidden replacement model.

## Prioritized future work

| Priority | Work | Approach that preserves capabilities |
| --- | --- | --- |
| 1 | Establish behavior coverage | Expand Serde round-trip/mutation tests, generator argument and custom-JAR integration tests, and isolated tool lifecycle tests before implementation changes |
| 2 | Complete the schema model | Add missing primitive aliases, recursive groups, message data, offsets, block lengths and custom-header handling without replacing the typed API or serialization support; prove old XML output still works |
| 3 | Preserve declaration order | Investigate a compatible representation for mixed composite/body elements; keep public fields and serialization behavior through adapters if internal changes are needed |
| 4 | Error handling | Introduce fallible loading entry points and contextual subprocess errors while retaining existing entry points and error conversions; explicitly review any changed panic/exit behavior |
| 5 | Real validation | Design diagnostics and test all 66 existing XML fixtures against a declared upstream version; review the behavior transition from the currently unfinished command |
| 6 | Complete evolution checks | Define reader directions, implement message/nested-group/type comparisons and actual strategies, and preserve exported strategy APIs. Review changes to existing version acceptance and result variants |
| 7 | Generation/tool improvements | Verify C#/Rust/Java/Go routing with appropriate supplied JARs, preserve independent package/namespace forwarding, and add optional backends, aliases and artifact installation without removing existing routes |
| 8 | Dependency modernization | Upgrade one dependency group at a time against contract tests. Inspect the semver fork before replacing it and verify both parsing and serialization |
| 9 | Packaging/release | Resolve the Git-semver publication issue: Cargo packaging replaces the Git source with registry semver, whose short-version parsing differs. Verify packaged behavior before publishing |
| 10 | Edition 2024 | Optional separate migration after compatibility tests; review resolver changes independently |

Other findings to investigate: Eq/Hash discrepancies involving header selection and type presence; hard-coded header names; limited VTable coverage; inaccessible nested/error types affecting downstream extension; numeric hash test portability; and platform behavior of Gradle/SSH handling. Record expected behavior before changing these.

## Verification gate

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-targets --all-features --locked
cargo test --workspace --doc --locked
cargo doc --workspace --no-deps --locked
```

Use the original APIs and CLI workflows as the reference. A compiler/version bump does not justify removing functionality. Breaking proposals must be described separately, including affected capabilities and a migration/compatibility approach, before implementation.
