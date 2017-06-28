
#![forbid(unsafe_code, missing_docs)]

//! Rust types for trinary and epistemic logic.
//!
//! A nice way to use the types in this crate is with a domain-specific type alias via `pub use`.
//! (For [esoteric reasons](https://github.com/rust-lang/rust/issues/26264), a simple typedef-style
//! type alias doesn't work, though this Rust limitation will eventually be removed.)



mod tristate;
mod kleene;
mod skeptic;


pub use tristate::TriState;
pub use kleene::Kleene;
pub use skeptic::Skeptic;
