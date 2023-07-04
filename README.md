# bigint
Rust crate for arbitrarily big integers, signed or unsigned.

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

Python tests are available to be run in the `pytest` framework. They are located
in the `tests` folder and should provide ample example usage.