/// Positional arguments.
#[derive(Debug, Clone)]
pub struct Arg {
    pub(crate) name: String,
    pub(crate) optional: bool,
    pub(crate) variadic: bool,
}

impl Arg {
    /// Create a new positional argument.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            optional: false,
            variadic: false,
        }
    }

    /// Mark the argument as optional.
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    /// Mark the argument as accepting one or more values.
    pub fn variadic(mut self) -> Self {
        self.variadic = true;
        self
    }

    pub(crate) fn render(&self) -> String {
        let suffix = if self.variadic {
            "..."
        } else if self.optional {
            "?"
        } else {
            ""
        };
        format!("{}{}", self.name, suffix)
    }
}
