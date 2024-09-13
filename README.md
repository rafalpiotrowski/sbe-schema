[![Crates.io](https://img.shields.io/crates/v/sbe-schema)](https://crates.io/crates/sbe-schema)
[![minimum rustc version](https://img.shields.io/badge/rustc-1.64+-green.svg)](https://blog.rust-lang.org/2019/11/07/Rust-1.64.0.html)

Master 
[![Build Status](https://github.com/rafalpiotrowski/sbe-schema/actions/workflows/main.yml/badge.svg?branch=master)](https://github.com/rafalpiotrowski/sbe-schema)


# Simple Binary Encoding (SBE) Schema

SBE stands for Simple Binary Encoding. You can read more about it here: [SBE](https://github.com/real-logic/simple-binary-encoding)

This repo has two projects.

1. sbe-schema - which is a library that is used by the sbe cli tool to work with sbe schemas
2. sbe - cli utility tool to work with sbe schemas

## sbe cli

Command line interface (cli) to manage work arround simple-binary-encoding schema files to:

* generate code in desired programming language
* validate schema
* check if evolution comply to desired compatibility level

### installation

```bash
cargo install --git https://github.com/rafalpiotrowski/sbe-schema
```

after sucessful installtion executing the following command

```bash
sbe
```

should produce following output

```bash
SBE schema tool

Usage: sbe <COMMAND>

Commands:
  schema  Work with SBE schema files: validate and generate code for different languages
  tool    Work with SBE source code. Clone, build, and copy jar file for later use in code generation and schema validation. Requires to have java installed and available in the PATH or specify the path to the java executable
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

License (See LICENSE file for full license)

-------------------------------------------
Licensed under either the MIT or Apache License, Version 2.0 at your option (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0
    https://choosealicense.com/licenses/mit/

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
