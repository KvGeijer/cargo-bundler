# cargo-bundler


This project aims to be able to bundle a whole Cargo project into a single source-code Rust file. This can be very useful when submitting a larger code base to some website, such as for competitive programming.

It stands on the shoulders of [rust-bundler-cp](https://github.com/Endle/rust-bundler-cp) and the earlier [rust-bundler](https://github.com/slava-sh/rust-bundler/), who both used the idea of traversing and modifying the syntax tree. However, this re-imagination aims at automating the handling of paths effected by the bundling, as well as conforming to more modern Rust, e.g. not using `extern crate <...>`. However, this is a WIP and [rust-bundler-cp](https://github.com/Endle/rust-bundler-cp) is good enough for most use cases.



## Features

1. Bundles the library (`lib.rs`) into a single module which is inserted into the bin target (`main.rs`).
2. Bundles the expanded bin target (`main.rs`) and its modules into a single file.
3. All use paths are adjusted to the new merged module tree.

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
* This project is similar to [rust-bundler-cp](https://github.com/Endle/rust-bundler-cp) which is based on [slava-sh/rust-bundler](https://github.com/slava-sh/rust-bundler),
* [lpenz/rust-sourcebundler](https://github.com/lpenz/rust-sourcebundler) which is similar to the above, but uses regular expressions instead of mutating the syntax tree,
* [MarcosCosmos/cg-rust-bundler](https://github.com/MarcosCosmos/cg-rust-bundler).
