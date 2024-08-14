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
cargo test
```

For benchmarks, please visit the `benches` folder.

# Performance
More details and scripts about performance are available in the `benches` 
folder.

TL;DR -> The current state of `twibint`s performance (v0.2.7) is: Addition, 
Subtraction and Multiplication are faster than for Python integers, and faster 
then `num-bigint` at some scales. Division remains extremely slow.

# List of features

- `rand`: exports the function `gen_random_biguint`: enables the possibility to generate 
a random integer with a specific number of bits. Uses `rand` crate as a dependency.
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
in the `tests` folder and should provide ample example usage. Run the tests with 
```python
pytest tests
```

Performance comparison with Python's default integers are available in the
`benches` folder.


# Changelog for version 0.2
This new version contains extensive accelerations for addition, subtraction, and 
multiplication on x86_64 machines. I used no modern extensions of x86, so these 
acceleration should be portable accross this family of machines. These 
will probably also have performance repercussions on many other features.

These acceleration are mostly due to dropping inline assembly for core loops, and are 
based on `unsafe` Rust. Other `unsafe` features used include smartly swapping between 
`&[u32]` and `&[u64]` slices via pointers (when alignment is lucky).

To disable any `unsafe` code, use with the flag `--no-default-features` for a slower
experience, but fully compiled in safe Rust.