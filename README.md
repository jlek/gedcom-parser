# Gedcom Parser

This submission is not complete: it only works for the single-node Gedcom file. (I hope.)

## To Run

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Build: `cargo build --release`
3. `target/release/gedcom-parser SOURCE_FILE_PATH TARGET_FILE_PATH` (_e.g._ `target/release/gedcom-parser src/tests/one-node.ged src/tests/one-node.json`)

## To Run the Tests

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. `cargo test`
