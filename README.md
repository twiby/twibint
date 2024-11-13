# twibint
Rust crate for arbitrarily big integers, signed or unsigned.

[![crate](https://img.shields.io/crates/v/twibint.svg)](https://crates.io/crates/twibint)
[![documentation](https://docs.rs/twibint/badge.svg)](https://docs.rs/twibint)

The main API of this crate is to export 2 types: BigUint and BigInt, 
meant to represent unsigned or signed integers of arbitrarily large
absolute value. They are meant to be used in almost any way a regular 
integer can be used, though they don't implement the `Copy` trait.

These types let you choose the underlying digit representation (`u32` 
or `u64`) via a generic type parameter. All features are identical. 
You're welcome to try both to see what's most efficient for your use 
case on your particular machine. Typical usage include defining `type 
BigUint = BigUint<u64>;` at the beginning of your project.

Each integer can also be saved to, or imported from, a file, using 
`BigUint::write_to_file`, or `Imported::read_from_file`. The import 
creates an enum with one variant per available integer type. See 
documentation for more info.

Build, documentation, and tests are available the usual way calling
the following:

```bash
cargo build
cargo docs
cargo test
```

For benchmarks, please visit the `benches` folder.

This crate relies only on the standard library for its core features. 
Some dependencies are optional, depending on a specific feature (see 
list of features below).

# Performance
More details and scripts about performance are available in the `benches` 
folder.

TL;DR -> The current state of `twibint`s performance (v0.3.0) is: 
Addition, Subtraction, Multiplication, and Division are faster than 
for Python integers. Compared to `num-bigint`, Addition is mostly 
faster, Multiplication is equivalent, Subtraction and Division slightly 
worse. 

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


# Notice for version 0.2
This contains hardware acceleration for x86_64 architectures that use 
unsafe code. Forusers wanting extra security (i.e. only safe Rust), 
use with the flag `--no-default-features` for a slower experience, but 
fully compiled in safe Rust.

# Notice for version 0.3
This contains new algorithms for division that make this crate more 
usable. Additionally starts supporting exporting integers to file 
and importing them from a file. 