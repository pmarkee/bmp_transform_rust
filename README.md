# BMP transformer re-written in Rust

Former university assignment re-written in Rust for learning and fun.

[Original project](https://github.com/pmarkee/bmp_transform/)

## Usage

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. `cargo run -- <arguments>`

## Command line arguments

- `-i`, `--input` path to the input file. Defaults to `resources/in.bmp`
- `-o`, `--output` path to the output file. Defaults to `resources/out.bmp`.
- `-c`, `--color` what color to use. Possible values: `grey`, `red`, `green`, `blue`, `yellow`, `violet`, `cyan`.
