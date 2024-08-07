# twibint
Rust crate for arbitrarily big integers, signed or unsigned.

[![crate](https://img.shields.io/crates/v/twibint.svg)](https://crates.io/crates/twibint)
[![documentation](https://docs.rs/twibint/badge.svg)](https://docs.rs/twibint)

This crate does not need any dependency, and relies only on the standard 
library. Some dependencies are optional, depending on a specific feature (see 
list of features below).

The main API of this crate is to export 2 types: BigUint and BigInt, 
meant to represent unsigned or signed integers of arbitrarily large
absolute value. They are meant to be used in almost any way a regular 
integer can be used, though they don't implement the `Copy` trait.

Build, documentation, benchmarks and tests are available the usual way calling
the following:

```bash
cargo build
cargo docs
cargo bench --features=rand
cargo test
```

For benchmarks specifically, you might want to call only some of these:
```bash
cargo bench mul --features=rand
cargo bench add --features=rand
cargo bench sub --features=rand
```

Benchmarks won't compile/run without the `rand` feature enabled.

# Performance
The ambitious and naive goal of this project is to be as performant or better than 
any state of the art crate on these methods.

I choose to compare myself to `num-bigint` first, as it's quite standard at this 
point.

Today, on x86, `twibint` is faster than `num-bigint` v0.4 for addition, above 
around 10000 bits. It is on par for multiplication, starting 1000 bits. 

# List of features

- `rand`: enables the possibility to generate a random integer with a specific 
number of bits. Uses `rand` crate as a dependency.
- `pyo3`: Only used to generate python bindings, it's only meant to be used
indirectly via the `pip install .` command. Uses `pyo3` crate as a dependency.
- `unsafe`: Enables accelerations that use unsafe Rust. Enabled by default. 
Disabled via the flag `--no-default-features` for pure safe Rust (compile time enforced).


# Install as a Python package
Simply use from the base directory
```bash
python3 -m pip install .
```

This crate seems faster than the default Python integers for addition and multiplication
above a certain numbers of bits (between 1000 and 10000 bits).

Python tests are available to be run in the `pytest` framework. They are located
in the `tests` folder and should provide ample example usage.


# Changelog for version 0.2
This new version contains extensive accelerations for addition, subtraction, and 
multiplication on x86_64 machines. I used no modern extensions of x86, so these 
acceleration should be portable accross this family of mahcines. These will probably also 
have performance repercussions on many other features.

These acceleration are mostly due to dropping inline assembly for core loops, and are 
based on `unsafe` Rust. Other `unsafe` features used include smartly swapping between 
`&[u32]` and `&[u64]` slices via pointers (when alignment is lucky).

To disable any `unsafe` code, use with the flag `--no-default-features` for a slower
experience, but fully compiled in safe Rust.