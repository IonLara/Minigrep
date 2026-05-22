# minigrep

A grep-like CLI tool built in Rust. Base created by following the Book of Rust, but implemented extra functionality.

## Features
- Search for multiple terms using the '|' separator
- Case insensitive mode via 'IGNORE_CASE' env var
- Line numbers on results
- Colored match highlighting

## Usage
cargo run -- "foo|bar" file.txt
IGNORE_CASE=1 cargo run -- "foo|bar" file.txt
OR
cargo run "foo|bar" file.txt ci
