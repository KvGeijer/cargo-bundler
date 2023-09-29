# cargo-bundler


This tool bundles a whole Cargo project into a single Rust source-code file. This can be very useful when submitting a larger code-base to some websites, such as for competitive programming.

There are similar earlier tools, but they were older, requiring `extern crate ...`, and did not adjust paths/use paths to suit the new module structure. It uses a similar idea to [rust-bundler-cp](https://github.com/Endle/rust-bundler-cp) and the earlier [rust-bundler](https://github.com/slava-sh/rust-bundler/) of traversing and modifying the syntax tree.



## Features

1. Bundles the library (`lib.rs`) into a single module which is inserted into the bin target (`main.rs`).
2. Bundles the flattened bin target (`main.rs`) and its modules into a single file.
3. All use paths are adjusted to suit the new merged module tree.

* Uses [Syn](https://docs.rs/syn/latest/syn/) to parse and manipulate syntax trees.
* Handles all code within the crate, but does not link to other crates such as dependencies or sub-crates.

## Usage

Install:
```sh
$ cargo install --path .
```

Run:
```sh
$ cargo-bundler
```
which is equivalent to:
```sh
$ cargo-bundler --input . --binary main
```



## Similar Projects
* This project is similar to [endle/rust-bundler-cp](https://github.com/Endle/rust-bundler-cp) which is based on [slava-sh/rust-bundler](https://github.com/slava-sh/rust-bundler),
* [lpenz/rust-sourcebundler](https://github.com/lpenz/rust-sourcebundler) which is similar to the above, but uses regular expressions instead of mutating the syntax tree,
* [MarcosCosmos/cg-rust-bundler](https://github.com/MarcosCosmos/cg-rust-bundler).
