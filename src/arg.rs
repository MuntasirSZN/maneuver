/// Positional arguments.
#[derive(Debug, Clone)]
pub struct Arg {
    pub(crate) name: String,
}

impl Arg {
    /// Create a new positional argument.
    pub fn new(name: &str) -> Self {
        Self { name: name.into() }
    }
}
