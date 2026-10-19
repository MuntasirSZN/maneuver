/// Add a custom section
#[derive(Debug, Clone)]
pub struct Section {
    pub(crate) name: String,
    pub(crate) paragraphs: Vec<String>,
}

impl Section {
    /// Create a new section with the given name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            paragraphs: vec![],
        }
    }

    /// Add a paragraph to the section
    pub fn paragraph(mut self, text: &str) -> Self {
        self.paragraphs.push(text.into());
        self
    }
}
