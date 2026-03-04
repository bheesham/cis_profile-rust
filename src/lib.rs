// DEBT: we slow down the compiler in favour of maintaining downstream
// compatibility.
//
// Addresses:
//  the `Err`-variant returned from this function is very large
// https://rust-lang.github.io/rust-clippy/rust-1.93.0/index.html#result_large_err
#![allow(clippy::result_large_err)]

pub mod crypto;
pub mod error;
pub mod filter;
pub mod keys;
pub mod schema;
pub mod utils;
