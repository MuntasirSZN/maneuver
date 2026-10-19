/// Boolean arguments that can be toggled on or off.
#[derive(Debug, Clone, Default)]
pub struct Flag {
    pub(crate) short: Option<String>,
    pub(crate) long: Option<String>,
    pub(crate) help: Option<String>,
}

impl Flag {
    /// Create a new instance.
    pub fn new() -> Self {
        Self::default()
    }
}

impl_help!(Flag);
impl_short_long!(Flag);
