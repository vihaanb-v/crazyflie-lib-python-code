# RTLola StreamIR

[![Crate](https://img.shields.io/crates/v/rtlola-streamir.svg)](https://crates.io/crates/rtlola-streamir)
[![API](https://docs.rs/rtlola-streamir/badge.svg)](https://docs.rs/rtlola-streamir)
[![License](https://img.shields.io/crates/l/rtlola-streamir)](https://crates.io/crates/rtlola-streamir)

RTLola is a stream-based runtime verification framework.
This crate contains common functionality for all tools working with the intermediate representation StreamIR.
It defines the rewriting rules for optimization of the StreamIR and provides a mechanism for applying the rules until a fixed point is reached.
Furthermore, it provides a framework for easily translating the StreamIR into target languages.

The following crates build upon this StreamIR framework:
* [rtlola2rust](https://crates.io/crates/rtlola2rust): A compilation from RTLola to Rust
* [rtlola2solidity](https://crates.io/crates/rtlola2solidity): A compilation from RTLola to Solidity
* [rtlola-streamir-interpreter](https://crates.io/crates/rtlola-streamir-interpreter): A interpretation of the StreamIR through JIT compilation 

For more information about StreamIR we refer the the [accompaning paper](https://arxiv.org/abs/2504.21458).

For more information on the RTLola framework make sure to visit our Website: rtlola.org

## Binary
The `rtlola-streamir` provides a binary for displaying and debugging the resulting StreamIR when implementing new rewriting rules.
The StreamIR is represented with parallel statements stacked horizontally, while sequential statements are stacked vertically.
Simply run the binary with the specification file:
```
$ rtlola-streamir test.lola
if @a then
    shift a
    -------
    input a
-----------------------------------------------------
if @a then  | if @a then  | if @a then
    shift b |     shift c |     shift d
-----------------------------------------------------
if @a then                | if @a then
    eval_0 b with (a()+1) |     eval_0 c with (a()-1)
-----------------------------------------------------
if @a then
    eval_0 d with (b()+c())
```
and add the `--optimize-all` argument for displaying the StreamIR after applying all rewriting rules:
```
$ rtlola-streamir test.lola --optimize-all
if @a then
    input a
    ---------------------------------------------
    eval_0 b with (a()+1) | eval_0 c with (a()-1)
    ---------------------------------------------
    eval_0 d with (b()+c())
```

# Copyright

Copyright (C) CISPA - Helmholtz Center for Information Security 2024-2025. Authors: Jan Baumeister, Frederik Scheerer
