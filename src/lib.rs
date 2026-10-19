#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

macro_rules! impl_help {
    ($type:ty) => {
        impl $type {
            /// Set the help text.
            pub fn help(mut self, help: &str) -> Self {
                self.help = Some(help.into());
                self
            }
        }
    };
}

macro_rules! impl_default_value {
    ($type:ty) => {
        impl $type {
            /// Set the default value.
            pub fn default_value(mut self, default: &str) -> Self {
                self.default = Some(default.into());
                self
            }
        }
    };
}

macro_rules! impl_short_long {
    ($type:ty) => {
        impl $type {
            /// Set the short option.
            pub fn short(mut self, short: &str) -> Self {
                self.short = Some(short.into());
                self
            }

            /// Set the long option.
            pub fn long(mut self, long: &str) -> Self {
                self.long = Some(long.into());
                self
            }
        }
    };
}

mod arg;
mod author;
mod command;
mod environment;
mod example;
mod flag;
mod man;
mod option;
mod section;

pub mod prelude;

pub use crate::arg::Arg;
pub use crate::author::Author;
pub use crate::command::Command;
pub use crate::environment::Env;
pub use crate::example::Example;
pub use crate::flag::Flag;
pub use crate::man::{ExitStatus, IntoArgs, Manual};
pub use crate::option::Opt;
pub use crate::section::Section;
