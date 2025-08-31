# RTLola to Solidity Compiler
[![Crate](https://img.shields.io/crates/v/rtlola2solidity.svg)](https://crates.io/crates/rtlola2solidity)
[![API](https://docs.rs/rtlola2solidity/badge.svg)](https://docs.rs/rtlola2solidity)
[![License](https://img.shields.io/crates/l/rtlola2solidity)](https://crates.io/crates/rtlola2solidity)

RTLola is a stream-based runtime verification framework.
This crate provides a compilation of RTLola to Solidity through the use of the [StreamIR framework](https://crates.io/crates/rtlola-streamir).

For more information about StreamIR we refer to the [accompaning paper](https://arxiv.org/abs/2504.21458).

For more information on the RTLola framework make sure to visit our website: [rtlola.org](https://rtlola.org)

For detailed usage instructions try:
```
rtlola2solidity --help
```
It takes as input the path to an rtlola specification and a specification of the function interface in TOML.
Each function is represented by a `[[function]]` block with an associated name, and contains a set of `[[function.argument]]`'s, which represent the arguments to this function and provides inputs to the associated input streams.


# Copyright

Copyright (C) CISPA - Helmholtz Center for Information Security 2024-2025. Authors: Jan Baumeister, Frederik Scheerer