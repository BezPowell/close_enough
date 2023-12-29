# close_enough
[![Rust](https://github.com/BezPowell/close_enough/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/BezPowell/close_enough/actions/workflows/rust.yml)

Occasionally, returning a result that is 'close enough' is more useful than returning no result at all. This crate provides utilities for finding the difference between two values, and for getting the closest match from a set.

It is currently very much of a work in progress, and is almost certainly not the most efficient implementation.

## TODO!
- Document!
- Finish adding implementations of `Diff` for all of Rust's integer primitives.