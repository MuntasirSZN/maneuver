//! Convenience wrapper to import all the essential structs.
//!
//! ```rust
//! extern crate man;
//!
//! use man::prelude::*;
//!
//! fn main () {
//!   let msg = Manual::new("my-app").render();
//! }
//! ```
pub use crate::arg::Arg;
pub use crate::author::Author;
pub use crate::environment::Env;
pub use crate::example::Example;
pub use crate::flag::Flag;
pub use crate::man::Manual;
pub use crate::option::Opt;
pub use crate::section::Section;
