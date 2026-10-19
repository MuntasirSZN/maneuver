/// Application authors.
#[derive(Debug, Clone)]
pub struct Author {
    pub(crate) name: String,
    pub(crate) email: Option<String>,
    pub(crate) text: Vec<String>,
}

impl Author {
    /// Create a new instance.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            email: None,
            text: vec![],
        }
    }

    /// Set the email field.
    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.into());
        self
    }

    /// Add additional text to the author entry.
    pub fn text(mut self, text: &str) -> Self {
        self.text.push(text.into());
        self
    }
}
