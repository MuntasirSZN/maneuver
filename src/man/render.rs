use super::*;
use roff::{Inline, Roff, bold, italic, roman};

pub(super) fn render(manual: Manual) -> String {
    let mut page = Roff::new();
    let date = manual.date.as_deref();
    let version = manual.version.as_deref();
    let source = manual.source.as_deref();
    let mut header = vec![manual.name.to_uppercase(), "1".into()];
    if date.is_some() || version.is_some() || source.is_some() {
        header.push(date.unwrap_or("\"\"").into());
    }
    if version.is_some() || source.is_some() {
        header.push(version.unwrap_or("\"\"").into());
    }
    if let Some(source) = source {
        header.push(source.into());
    }
    page.control("TH", header.iter().map(String::as_str));
    page = about(page, &manual.name, &manual.about);
    page = synopsis(
        page,
        &manual.name,
        &manual.flags,
        &manual.options,
        &manual.arguments,
        !manual.commands.is_empty(),
    );
    page = description(page, &manual.description);
    page = commands(page, &manual.commands);
    page = flags(page, &manual.flags);
    page = options(page, &manual.options);
    page = arguments(page, &manual.arguments);
    page = env(page, &manual.environment);
    for section in manual.custom_sections {
        page = custom(page, section);
    }
    page = exit_status(page, manual.statuses.as_deref());
    page = examples(page, &manual.examples);
    page = authors(page, &manual.authors);
    page.render()
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
        Some(desc) => format!("{} - {}", name, desc),
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
fn synopsis(
    page: Roff,
    name: &str,
    flags: &[Flag],
    options: &[Opt],
    args: &[Arg],
    has_commands: bool,
) -> Roff {
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

    if has_commands {
        msg.push(roman(" [COMMAND]"));
    }

    section(page, "SYNOPSIS", msg)
}

/// Create a `COMMANDS` section.
///
/// Each command is rendered as a tagged paragraph holding its synopsis and
/// description. The options, flags and arguments a command accepts are nested
/// below it.
///
/// ## Formatting
/// ```txt
/// COMMANDS
///        check [OPTIONS] [PATH]
///               Check availability for a file or directory.
///        .RS 4
///        .TP
///        --all-targets
///               Run the check for all targets.
///        .RE
/// ```
fn commands(page: Roff, commands: &[Command]) -> Roff {
    if commands.is_empty() {
        return page;
    }

    let mut page = section(page, "COMMANDS", vec![]);
    for command in commands {
        page.control("TP", []);
        let details = command.details();
        if details.is_empty() {
            page.text([bold(&command.name)]);
        } else {
            page.text([bold(&command.name), roman(details)]);
        }
        if let Some(help) = &command.help {
            page.text([roman(help)]);
        }
        page = command_entries(page, command);
    }
    page
}

/// Renders the options, flags and arguments a command accepts.
fn command_entries(mut page: Roff, command: &Command) -> Roff {
    if command.options.is_empty() && command.flags.is_empty() && command.arguments.is_empty() {
        return page;
    }

    page.control("RS", ["4"]);
    page = definition_list(
        page,
        command
            .flags
            .iter()
            .map(|flag| (flag_args(flag), flag.help.as_deref().unwrap_or(""))),
    );
    page = definition_list(
        page,
        command
            .options
            .iter()
            .map(|opt| (option_args(opt), opt.help.as_deref().unwrap_or(""))),
    );
    page = definition_list(
        page,
        command
            .arguments
            .iter()
            .map(|arg| (vec![italic(&arg.name)], arg.help.as_deref().unwrap_or(""))),
    );
    page.control("RE", []);
    page
}

/// Create an `ARGUMENTS` section.
///
/// ## Formatting
/// ```txt
/// ARGUMENTS
///        PATH
///               The file to read.
/// ```
fn arguments(page: Roff, args: &[Arg]) -> Roff {
    if args.is_empty() {
        return page;
    }

    let entries = args
        .iter()
        .map(|arg| (vec![italic(&arg.name)], arg.help.as_deref().unwrap_or("")));
    definition_section(page, "ARGUMENTS", entries)
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

    let entries = flags
        .iter()
        .map(|flag| (flag_args(flag), flag.help.as_deref().unwrap_or("")));
    definition_section(page, "FLAGS", entries)
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

    let entries = options
        .iter()
        .map(|opt| (option_args(opt), opt.help.as_deref().unwrap_or("")));
    definition_section(page, "OPTIONS", entries)
}

/// Renders the inline arguments of a flag.
fn flag_args(flag: &Flag) -> Vec<Inline> {
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
    args
}

/// Renders the inline arguments of an option.
fn option_args(opt: &Opt) -> Vec<Inline> {
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
    args.push(roman(&opt.separator));
    args.push(italic(&opt.name));
    if let Some(ref default) = opt.default {
        args.push(roman(" "));
        args.push(roman("[default: "));
        args.push(italic(default));
        args.push(roman("]"));
    }
    args
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
    page: Roff,
    title: &str,
    entries: impl Iterator<Item = (Vec<Inline>, &'a str)>,
) -> Roff {
    let page = section(page, title, vec![]);
    definition_list(page, entries)
}

/// Emits a definition list.
///
/// Every entry becomes a tagged paragraph holding the arguments and their
/// description. The description is omitted when it is empty.
fn definition_list<'a>(
    mut page: Roff,
    entries: impl Iterator<Item = (Vec<Inline>, &'a str)>,
) -> Roff {
    for (args, help) in entries {
        page.control("TP", []);
        page.text(args);
        if !help.is_empty() {
            page.text([roman(help)]);
        }
    }
    page
}

/// Create a `EXIT STATUS` section.
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
fn exit_status(page: Roff, statuses: Option<&[ExitStatus]>) -> Roff {
    if matches!(statuses, Some(statuses) if statuses.is_empty()) {
        return page;
    }
    let defaults;
    let statuses = if let Some(statuses) = statuses {
        statuses
    } else {
        defaults = vec![
            ExitStatus::new("0", "Successful program execution."),
            ExitStatus::new("1", "Unsuccessful program execution."),
            ExitStatus::new("101", "The program panicked."),
        ];
        &defaults
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
