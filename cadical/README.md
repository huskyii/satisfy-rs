High-level Rust binding for Cadical
================================================================================

This crate provides Rust bindings for Cadical, an open source SAT solver,
written in C++ by [Armin Biere](https://github.com/arminbiere).
You can find more information about it [here](http://fmv.jku.at/cadical/), and
its source code available [here](https://github.com/arminbiere/cadical).

This crate strive to providing a hard to misuse and safe Rust API. It achieves
this goal with the help of Rust's powerful type system and move semantic.
I strongly encourge you reading the source code, it's only a few hundreds line
of code and I promise you will fall in love with Rust's type system. :P No. I'm
kidding, you are not gonna to fall in love with Rust, at least not before Enum's
variants become first-class type.

As [RFC 2593](https://github.com/rust-lang/rfcs/pull/2593) was not accepted yet,
API implemented here was not perfect.


Usage
--------------------------------------------------------------------------------
Currently, there's no formal document and tests in [tests directory](tests)
serve as document. Every test are numbered and I suggest reading them from 
[0_basic.rs](tests/0_basic.rs). 


Notes
--------------------------------------------------------------------------------
This crate binds to vendoring Cadical 1.2.1 release.

This crate doesn't expose some API(e.g. limit, set_options, etc), if you need thoes
API, you can get raw pointer via Cadical::raw_ptr() and use low-level API from
[cadical-sys](../cadical-sys).


License
--------------------------------------------------------------------------------
BSD 3-Clause