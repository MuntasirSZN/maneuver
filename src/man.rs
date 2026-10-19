use super::*;
use roff::{bold, italic, roman, Inline, Roff};

/// The main man page struct.
#[derive(Debug, Clone)]
pub struct ExitStatus {
    code: String,
    description: String,
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
    statuses: Vec<ExitStatus>,
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
            statuses: vec![],
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
        self.statuses.push(status);
        self
    }

    /// Add a positional argument. Items are displayed in insertion order.
    pub fn arg(mut self, arg: Arg) -> Self {
        self.arguments.push(arg);
        self
    }

    /// Render to a string.
    pub fn render(self) -> String {
        let mut page = Roff::new();
        page.control("TH", [self.name.to_uppercase().as_str(), "1"]);
        page = about(page, &self.name, &self.about);
        page = synopsis(
            page,
            &self.name,
            &self.flags,
            &self.options,
            &self.arguments,
        );
        page = description(page, &self.description);
        page = flags(page, &self.flags);
        page = options(page, &self.options);
        page = env(page, &self.environment);
        for section in self.custom_sections.into_iter() {
            page = custom(page, section);
        }
        page = exit_status(page, &self.statuses);
        page = examples(page, &self.examples);
        page = authors(page, &self.authors);
        page.render()
    }
}

/// Create a `NAME` section.
///
/// ## Formatting
/// ```txt
/// NAME
///         mycmd - brief help of the application
/// ```
fn about(page: Roff, name: &str, desc: &Option<String>) -> Roff {
    let desc = match desc {
        Some(ref desc) => format!("{} - {}", name, desc),
        None => name.to_owned(),
    };

    section(page, "NAME", vec![roman(desc)])
}

/// Create a `DESCRIPTION` section.
///
/// ## Formatting
/// ```txt
/// DESCRIPTION
///         Very long description of the application
/// ```
fn description(page: Roff, desc: &Option<String>) -> Roff {
    if let Some(desc) = desc {
        section(page, "DESCRIPTION", vec![roman(desc)])
    } else {
        page
    }
}

/// Create a `SYNOPSIS` section.
fn synopsis(page: Roff, name: &str, flags: &[Flag], options: &[Opt], args: &[Arg]) -> Roff {
    let mut msg: Vec<Inline> = vec![bold(name)];
    let flags = match flags.len() {
        0 => "",
        _ => " [FLAGS]",
    };
    let options = match options.len() {
        0 => "",
        _ => " [OPTIONS]",
    };
    msg.push(roman(flags));
    msg.push(roman(options));

    for arg in args {
        msg.push(roman(format!(" {}", arg.render())));
    }

    section(page, "SYNOPSIS", msg)
}

/// Create a `AUTHOR` or `AUTHORS` section.
///
/// ## Formatting
/// ```txt
/// AUTHORS
///          Alice Person <alice@person.com>
///          Bob Human <bob@human.com>
/// ```
fn authors(page: Roff, authors: &[Author]) -> Roff {
    let title = match authors.len() {
        0 => return page,
        1 => "AUTHOR",
        _ => "AUTHORS",
    };

    let mut page = page;
    page.control("SH", [title]);
    init_list(&mut page);
    for author in authors {
        let mut value = vec![roman(author.name.to_owned())];
        if let Some(ref email) = author.email {
            value.push(roman(format!(" <{}>", email)))
        };
        for text in &author.text {
            value.push(roman(format!(" {}", text)))
        }
        page.text(value);
    }
    page
}

/// Create a `FLAGS` section.
///
/// ## Formatting
/// ```txt
/// FLAGS
/// ```
fn flags(page: Roff, flags: &[Flag]) -> Roff {
    if flags.is_empty() {
        return page;
    }

    let mut page = section(page, "FLAGS", vec![]);
    for flag in flags {
        let mut args: Vec<Inline> = vec![];
        if let Some(ref short) = flag.short {
            args.push(bold(short));
        }
        if let Some(ref long) = flag.long {
            if !args.is_empty() {
                args.push(roman(", "));
            }
            args.push(bold(long));
        }
        page.control("TP", []);
        page.text(args);
        page.text([roman(flag.help.as_deref().unwrap_or(""))]);
    }
    page
}

/// Create a `OPTIONS` section.
///
/// ## Formatting
/// ```txt
/// OPTIONS
/// ```
fn options(page: Roff, options: &[Opt]) -> Roff {
    if options.is_empty() {
        return page;
    }

    let entries = options.iter().map(|opt| {
        let mut args: Vec<Inline> = vec![];
        if let Some(ref short) = opt.short {
            args.push(bold(short));
        }
        if let Some(ref long) = opt.long {
            if !args.is_empty() {
                args.push(roman(", "));
            }
            args.push(bold(long));
        }
        args.push(roman("="));
        args.push(italic(&opt.name));
        if let Some(ref default) = opt.default {
            args.push(roman(" "));
            args.push(roman("[default: "));
            args.push(italic(default));
            args.push(roman("]"));
        }
        (args, opt.help.as_deref().unwrap_or(""))
    });
    definition_section(page, "OPTIONS", entries)
}

/// Create a `ENVIRONMENT` section.
///
/// ## Formatting
///
/// ```txt
/// ENVIRONMENT
/// ```
fn env(page: Roff, environment: &[Env]) -> Roff {
    if environment.is_empty() {
        return page;
    }

    let entries = environment.iter().map(|env| {
        let mut args: Vec<Inline> = vec![bold(&env.name)];
        if let Some(ref default) = env.default {
            args.push(roman(" "));
            args.push(roman("[default: "));
            args.push(italic(default));
            args.push(roman("]"));
        }
        (args, env.help.as_deref().unwrap_or(""))
    });
    definition_section(page, "ENVIRONMENT", entries)
}

fn definition_section<'a>(
    mut page: Roff,
    title: &str,
    entries: impl Iterator<Item = (Vec<Inline>, &'a str)>,
) -> Roff {
    page = section(page, title, vec![]);
    for (args, help) in entries {
        page.control("TP", []);
        page.text(args);
        page.text([roman(help)]);
    }
    page
}

/// Create a `EXIT STATUS` section.
///
/// ## Implementation Note
/// This currently only returns the status code `0`, and takes no arguments. We
/// should let it take arguments.
///
/// ## Formatting
/// ```txt
/// EXIT STATUS
///        0      Successful program execution
///
///        1      Usage, syntax or configuration file error
///
///        2      Optional error
/// ```
fn exit_status(page: Roff, statuses: &[ExitStatus]) -> Roff {
    let defaults;
    let statuses = if statuses.is_empty() {
        defaults = vec![
            ExitStatus::new("0", "Successful program execution."),
            ExitStatus::new("1", "Unsuccessful program execution."),
            ExitStatus::new("101", "The program panicked."),
        ];
        &defaults
    } else {
        statuses
    };
    let mut page = section(page, "EXIT STATUS", vec![]);
    for status in statuses {
        page.control("TP", []);
        page.text([bold(&status.code)]);
        page.text([roman(&status.description)]);
    }
    page
}

/// Create a custom section.
///
/// The custom section will have the title you specify as the argument to the
/// .new() method and may optionally be followed by one or more paragraphs
/// using the .paragraph() method.
///
/// ## Formatting
/// ```txt
/// SECTION NAME
///        Text of first paragraph
///
///        Text of second paragraph
///
/// ```
fn custom(page: Roff, custom_section: Section) -> Roff {
    let mut page = section(page, &custom_section.name, vec![]);
    let paragraph_count = custom_section.paragraphs.len();
    for (index, paragraph) in custom_section.paragraphs.into_iter().enumerate() {
        page.text([roman(paragraph)]);
        if index + 1 != paragraph_count {
            page.text([roman("\n")]);
        }
    }
    page
}

/// Create an examples section
///
/// examples can have text (shown before the example command) and the command
/// itself.  Optionally, you can also display the output of the command, but
/// this is typically not necessary.  You may also change the prompt displayed
/// before the command (the default is `$`).
///
/// The command is printed in bold.
///
/// ## Formatting
/// ```txt
/// EXAMPLES
///        Explanatory text
///        $ command
///        output
/// ```
fn examples(page: Roff, examples: &[Example]) -> Roff {
    if examples.is_empty() {
        return page;
    };
    let mut page = section(page, "EXAMPLES", vec![]);
    for example in examples {
        let text = example.text.unwrap_or("");
        let mut full_command = String::from(example.prompt);
        if let Some(command) = example.command {
            full_command.push(' ');
            full_command.push_str(command);
        };
        page.control("TP", []);
        page.text([roman(text)]);
        let mut command = vec![bold(full_command)];
        if let Some(output) = example.output {
            command.push(Inline::LineBreak);
            command.push(roman(output));
        } else {
            command.push(roman("\n"));
        }
        page.text(command);
    }
    page
}

fn init_list(page: &mut Roff) {
    page.control("P", []).control("RS", ["2"]).control("nf", []);
}

fn section(mut page: Roff, title: &str, first_line: Vec<Inline>) -> Roff {
    page.control("SH", [title]);
    if !first_line.is_empty() {
        page.text(first_line);
    }
    page
}
