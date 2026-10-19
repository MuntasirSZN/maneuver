/// Positional arguments.
#[derive(Debug, Clone)]
pub struct Arg {
    pub(crate) name: String,
    pub(crate) help: Option<String>,
    pub(crate) optional: bool,
    pub(crate) variadic: bool,
}

impl Arg {
    /// Create a new positional argument.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            help: None,
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

    /// Render the argument as it is spelled in a synopsis.
    pub(crate) fn render(&self) -> String {
        let mut rendered = if self.optional {
            format!("[{}]", self.name)
        } else {
            self.name.clone()
        };
        if self.variadic {
            rendered.push_str("...");
        }
        rendered
    }
}

impl_help!(Arg);
