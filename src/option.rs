/// Arguments that take values.
#[derive(Debug, Clone)]
pub struct Opt {
    pub(crate) name: String,
    pub(crate) default: Option<String>,
    pub(crate) help: Option<String>,
    pub(crate) short: Option<String>,
    pub(crate) long: Option<String>,
    pub(crate) separator: String,
}

impl Opt {
    /// Create a new instance.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            default: None,
            help: None,
            short: None,
            long: None,
            separator: "=".into(),
        }
    }
}

impl_default_value!(Opt);
impl_help!(Opt);
impl_short_long!(Opt);

impl Opt {
    /// Set the separator between an option name and its value.
    ///
    /// The default is `"="`; use `" "` for forms such as `--output FILE`.
    pub fn separator(mut self, separator: &str) -> Self {
        self.separator = separator.into();
        self
    }
}
