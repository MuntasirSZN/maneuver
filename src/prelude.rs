//! Convenience wrapper to import all the essential structs.
//!
//! ```rust
//! use man::prelude::*;
//!
//! fn main () {
//!   let msg = Manual::new("my-app").render();
//! }
//! ```
pub use crate::{Arg, Author, Env, Example, ExitStatus, Flag, Manual, Opt, Section};
