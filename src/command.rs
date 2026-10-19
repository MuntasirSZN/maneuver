use crate::{Arg, Flag, Opt};

/// A subcommand of the documented program.
#[derive(Debug, Clone)]
pub struct Command {
    pub(crate) name: String,
    pub(crate) help: Option<String>,
    pub(crate) options: Vec<Opt>,
    pub(crate) flags: Vec<Flag>,
    pub(crate) arguments: Vec<Arg>,
}

impl Command {
    /// Create a new command.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            help: None,
            options: vec![],
            flags: vec![],
            arguments: vec![],
        }
    }

    /// Add an option accepted by the command.
    pub fn option(mut self, option: Opt) -> Self {
        self.options.push(option);
        self
    }

    /// Add a flag accepted by the command.
    pub fn flag(mut self, flag: Flag) -> Self {
        self.flags.push(flag);
        self
    }

    /// Add a positional argument accepted by the command.
    pub fn argument(mut self, argument: Arg) -> Self {
        self.arguments.push(argument);
        self
    }

    /// Renders the part of the synopsis that follows the command name.
    ///
    /// The result is empty when the command takes no flags, options or
    /// arguments.
    pub(crate) fn details(&self) -> String {
        let mut details = String::new();
        if !self.flags.is_empty() {
            details.push_str(" [FLAGS]");
        }
        if !self.options.is_empty() {
            details.push_str(" [OPTIONS]");
        }
        for argument in &self.arguments {
            details.push(' ');
            details.push_str(&argument.render());
        }
        details
    }
}

impl_help!(Command);
