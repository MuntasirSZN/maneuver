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

pub use arg::Arg;
pub use author::Author;
pub use environment::Env;
pub use example::Example;
pub use flag::Flag;
pub use man::Manual;
pub use option::Opt;
pub use section::Section;
