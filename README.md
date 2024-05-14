# twibint
Rust crate for arbitrarily big integers, signed or unsigned.

[![crate](https://img.shields.io/crates/v/twibint.svg)](https://crates.io/crates/twibint)
[![documentation](https://docs.rs/twibint/badge.svg)](https://docs.rs/twibint)


The main API of this crate is to export 2 types: BigUint and BigInt, 
meant to represent unsigned or signed integers of arbitrarily large
absolute value. They are meant to be used in almost any way a regular 
integer can be used. The only caveat is that they don't implement the 
Copy trait ; this means that calling `A + B` for example will perform
a `move` operation on `A` and `B`, losing ownership of them. Most of the 
time you will actually want to call `&A + &B`, performing the operation
"by reference".

Build, documentation, benchmarks and tests are available the usual way calling
the following:

```bash
cargo build
cargo docs
cargo bench
cargo test
```

For benchmarks specifically, you might want to call only some of these:
```bash
cargo bench mul
cargo bench add
cargo bench sub
```

## Install as a Python package
Simply use from the base directory
```bash
python3 -m pip install .
```

or from the PyPi repository
```bash
pip install twibint
```

Python tests are available to be run in the `pytest` framework. They are located
in the `tests` folder and should provide ample example usage.

# Changelog for version 0.2
This new version contains extensive accelerations for addition, subtraction, and 
multiplication on x86_64 machines. These will probably also have performance 
repercussions on many other features.

These acceleration are mostly due to dropping inline assembly for core loops, and are 
based on `unsafe` Rust. Other `unsafe` features used include smartly swapping between 
`&[u32]` and `&[u64]` slices via pointers.