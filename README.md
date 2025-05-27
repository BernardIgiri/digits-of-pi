# Printing Digits of Pi in Rust

This repository is written as an example project to use for practicing optimizing code in Rust. This program outputs the digits of pi to the standard output. It can go on forever if necessary. The blog post [Profiling Rust Code](https://yequalscode.com/posts/profiling-rust-code) explains how you can profile this code. Follow the instructions there to find your own optimizations.

## Source Algorithm

This Rust implementation is based on a Python spigot algorithm for pi found in:

- https://github.com/transmogrifier/pidigits

## Usage

Run the program with:

```bash
cargo run
```

## Profiling

Profile the code with:

```bash
cargo build --profile profiling --features profiling
```

This generates the following files:

```text
profiling/
├── flamegraph.svg
└── profile.pb
```

You can use flamegraph.svg and profile.pg to see the profiling results.

## Testing

Run tests with:

```bash
cargo test
```

## Blog

For more information refer to the [Y = Code](https://yequalscode.com) blog post at: https://yequalscode.com/posts/profiling-rust-code
