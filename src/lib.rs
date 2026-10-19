#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate roff;

mod arg;
mod author;
mod environment;
mod example;
mod flag;
mod man;
mod option;
mod section;

pub mod prelude;

pub use crate::arg::Arg;
pub use crate::author::Author;
pub use crate::environment::Env;
pub use crate::example::Example;
pub use crate::flag::Flag;
pub use crate::man::Manual;
pub use crate::option::Opt;
pub use crate::section::Section;
