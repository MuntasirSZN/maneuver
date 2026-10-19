/// Arguments that are passed by the executing shell.
#[derive(Debug, Clone)]
pub struct Env {
    pub(crate) name: String,
    pub(crate) default: Option<String>,
    pub(crate) help: Option<String>,
}

impl Env {
    /// Create a new instance.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            default: None,
            help: None,
        }
    }
}

impl_default_value!(Env);
impl_help!(Env);
