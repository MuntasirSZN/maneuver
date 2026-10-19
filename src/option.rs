/// Arguments that take values.
#[derive(Debug, Clone)]
pub struct Opt {
    pub(crate) name: String,
    pub(crate) default: Option<String>,
    pub(crate) help: Option<String>,
    pub(crate) short: Option<String>,
    pub(crate) long: Option<String>,
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
        }
    }
}

impl_default_value!(Opt);
impl_help!(Opt);
impl_short_long!(Opt);
