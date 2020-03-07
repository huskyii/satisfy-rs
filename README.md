Rust bindings to state of art SAT solvers
================================================================================

Back to summer of 2015, when I first met SAT solver, it's picosat written by
Armin Biere and used in BugScope, I'm impressed by the beauty of SAT solver.
And I'm also intrigued by Rust, which brings fancy state of art features that
come from recent programming language research to system programming world. I
believe Rust will make SAT solvers better. This project is my attempt to bring
state of art SAT solver to Rust ecosystem with safe API.

Currently, Armin Biere's Cadical is supported. You can find high-level binding
[here](cadical) and raw binding [here](cadical-sys).


TODO
--------------------------------------------------------------------------------
- support for minisat
- and other SAT solvers that derived from minisat
- Tseitin transformation?

License
--------------------------------------------------------------------------------
BSD 3-Clause

