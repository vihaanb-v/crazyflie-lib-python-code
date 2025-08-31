# RTLola to Rust Compiler
[![Crate](https://img.shields.io/crates/v/rtlola2rust.svg)](https://crates.io/crates/rtlola2rust)
[![API](https://docs.rs/rtlola2rust/badge.svg)](https://docs.rs/rtlola2rust)
[![License](https://img.shields.io/crates/l/rtlola2rust)](https://crates.io/crates/rtlola2rust)

RTLola is a stream-based runtime verification framework.
This crate provides a compiler from RTLola to Rust through the use of the [StreamIR library](https://crates.io/crates/rtlola-streamir).

For detailed usage instructions try:
```
rtlola2rust --help
```

For more information about StreamIR we refer to the [accompaning paper](https://arxiv.org/abs/2504.21458).

For more information on the RTLola framework make sure to visit our website: [rtlola.org](https://rtlola.org)

## Embedded Rust

Our compiler optionally allows to compile a `no_std` version of the monitor with the help of [heapless](https://crates.io/crates/heapless).
Use `--no-std` to make use of this feature, and specify the maximal number of instances per parameterized stream through the `--num-instances` argument:
```
$ rtlola2rust --no-std --num_instances a=10,b=50 test.lola
```

# Copyright

Copyright (C) CISPA - Helmholtz Center for Information Security 2024-2025. Authors: Jan Baumeister, Frederik Scheerer

