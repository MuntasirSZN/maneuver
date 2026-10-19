use super::*;
use std::path::PathBuf;

/// The main man page struct.
#[derive(Debug, Clone)]
pub struct ExitStatus {
    code: String,
    description: String,
}

/// Values accepted by [`Manual::arg`].
pub trait IntoArgs {
    /// Append positional arguments to a manual.
    fn append_to(self, arguments: &mut Vec<Arg>);
}

impl IntoArgs for Arg {
    fn append_to(self, arguments: &mut Vec<Arg>) {
        arguments.push(self);
    }
}

impl IntoArgs for Vec<Arg> {
    fn append_to(self, arguments: &mut Vec<Arg>) {
        arguments.extend(self);
    }
}

impl ExitStatus {
    /// Create an exit status entry.
    pub fn new(code: &str, description: &str) -> Self {
        Self {
            code: code.into(),
            description: description.into(),
        }
    }
}

/// The main man page struct.
#[derive(Debug, Clone)]
pub struct Manual {
    name: String,
    about: Option<String>,
    description: Option<String>,
    authors: Vec<Author>,
    flags: Vec<Flag>,
    options: Vec<Opt>,
    environment: Vec<Env>,
    arguments: Vec<Arg>,
    custom_sections: Vec<Section>,
    examples: Vec<Example>,
    statuses: Option<Vec<ExitStatus>>,
    version: Option<String>,
    date: Option<String>,
    source: Option<String>,
}

impl Manual {
    /// Create a new instance.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            about: None,
            description: None,
            authors: vec![],
            flags: vec![],
            options: vec![],
            arguments: vec![],
            environment: vec![],
            custom_sections: vec![],
            examples: vec![],
            statuses: None,
            version: Some(env!("CARGO_PKG_VERSION").into()),
            date: None,
            source: None,
        }
    }

    /// Add a short description.
    pub fn about<S: Into<String>>(mut self, about: S) -> Self {
        self.about = Some(about.into());
        self
    }

    /// Add a long description.
    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Add an author.
    pub fn author(mut self, author: Author) -> Self {
        self.authors.push(author);
        self
    }

    /// Add an environment variable.
    pub fn env(mut self, env: Env) -> Self {
        self.environment.push(env);
        self
    }

    /// Add an flag.
    pub fn flag(mut self, flag: Flag) -> Self {
        self.flags.push(flag);
        self
    }

    /// Add an option.
    pub fn option(mut self, opt: Opt) -> Self {
        self.options.push(opt);
        self
    }

    /// Add a custom section
    pub fn custom(mut self, custom_section: Section) -> Self {
        self.custom_sections.push(custom_section);
        self
    }

    /// Add an examples section
    pub fn example(mut self, example: Example) -> Self {
        self.examples.push(example);
        self
    }

    /// Add an exit status entry.
    pub fn status(mut self, status: ExitStatus) -> Self {
        self.statuses.get_or_insert_with(Vec::new).push(status);
        self
    }

    /// Omit the EXIT STATUS section.
    pub fn without_exit_status(mut self) -> Self {
        self.statuses = Some(vec![]);
        self
    }

    /// Set the version displayed by the man-page formatter.
    pub fn version<S: Into<String>>(mut self, version: S) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Set the date displayed by the man-page formatter.
    pub fn date<S: Into<String>>(mut self, date: S) -> Self {
        self.date = Some(date.into());
        self
    }

    /// Set the source or project identifier displayed by the formatter.
    pub fn source<S: Into<String>>(mut self, source: S) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Disable the version field.
    pub fn without_version(mut self) -> Self {
        self.version = None;
        self
    }

    /// Return the conventional filename for this manual page.
    pub fn path(&self) -> PathBuf {
        PathBuf::from(format!("{}.1", self.name))
    }

    /// Add a positional argument. Items are displayed in insertion order.
    pub fn arg<A: IntoArgs>(mut self, args: A) -> Self {
        args.append_to(&mut self.arguments);
        self
    }

    /// Render to a string.
    pub fn render(self) -> String {
        render::render(self)
    }
}

mod render;
