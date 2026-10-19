use insta::assert_snapshot;
use man::{Arg, Author, Env, Example, ExitStatus, Flag, Manual, Opt, Section};

#[test]
fn empty_manual_contains_required_sections() {
    assert_snapshot!(Manual::new("demo").render());
}

#[test]
fn manual_renders_description_and_positional_arguments() {
    assert_snapshot!(Manual::new("demo")
        .about("short description")
        .description("long description")
        .arg(Arg::new("input"))
        .arg(Arg::new("output"))
        .render());
}

#[test]
fn manual_renders_optional_and_variadic_arguments() {
    assert_snapshot!(Manual::new("demo")
        .arg(Arg::new("config").optional())
        .arg(Arg::new("files").variadic())
        .render());
}

#[test]
fn manual_renders_flags_with_all_field_combinations() {
    assert_snapshot!(Manual::new("demo")
        .flag(Flag::new().short("-s").long("--short").help("both names"))
        .flag(Flag::new().short("-s"))
        .flag(Flag::new().long("--long"))
        .flag(Flag::new().help("no names"))
        .render());
}

#[test]
fn manual_renders_options_with_defaults_and_optional_names() {
    assert_snapshot!(Manual::new("demo")
        .option(
            Opt::new("output")
                .short("-o")
                .long("--output")
                .default_value("stdout")
                .help("where to write"),
        )
        .option(Opt::new("input").short("-i"))
        .option(Opt::new("mode").long("--mode"))
        .option(Opt::new("count").help("number of items"))
        .render());
}

#[test]
fn manual_renders_environment_values_and_help() {
    assert_snapshot!(Manual::new("demo")
        .env(
            Env::new("PORT")
                .default_value("8080")
                .help("listening port"),
        )
        .env(Env::new("MODE").help("execution mode"))
        .env(Env::new("EMPTY"))
        .render());
}

#[test]
fn manual_renders_custom_sections_and_multiple_paragraphs() {
    assert_snapshot!(Manual::new("demo")
        .custom(
            Section::new("usage note")
                .paragraph("first paragraph")
                .paragraph("second paragraph"),
        )
        .render());
}

#[test]
fn manual_renders_examples_with_and_without_output() {
    assert_snapshot!(Manual::new("demo")
        .example(
            Example::new()
                .text("run in debug mode")
                .command("demo --debug")
                .output("debugging enabled"),
        )
        .example(
            Example::new()
                .prompt("#")
                .command("demo")
                .text("run as root"),
        )
        .example(Example::new().text("no command"))
        .render());
}

#[test]
fn manual_renders_single_and_multiple_authors() {
    assert_snapshot!(
        "single_author",
        Manual::new("demo")
            .author(Author::new("Alice").email("alice@example.com"))
            .render()
    );

    assert_snapshot!(
        "multiple_authors",
        Manual::new("demo")
            .author(Author::new("Alice"))
            .author(Author::new("Bob").email("bob@example.com"))
            .render()
    );

    assert_snapshot!(Manual::new("demo")
        .author(
            Author::new("Project Team")
                .email("team@example.com")
                .text("Maintained by the platform group.")
                .text("Contact us for contribution questions.")
        )
        .render());
}

#[test]
fn manual_renders_custom_exit_statuses() {
    assert_snapshot!(Manual::new("demo")
        .status(ExitStatus::new("64", "Command line usage error."))
        .status(ExitStatus::new("70", "Internal software error."))
        .render());
}

#[test]
fn manual_snapshot_covers_all_sections() {
    let page = Manual::new("auth-service")
        .about("authorize & authenticate members")
        .description("A command-line service for managing authenticated members.")
        .arg(Arg::new("path"))
        .env(
            Env::new("PORT")
                .default_value("3000")
                .help("The network port to listen to."),
        )
        .flag(
            Flag::new()
                .short("-v")
                .long("--verbose")
                .help("Enable verbose logging."),
        )
        .option(
            Opt::new("output")
                .short("-o")
                .long("--output")
                .default_value("stdout")
                .help("The output destination."),
        )
        .custom(
            Section::new("usage note")
                .paragraph("The service should be started before requests arrive.")
                .paragraph("Use the environment variable to select its port."),
        )
        .example(
            Example::new()
                .text("run the service")
                .command("auth-service --verbose")
                .output("now listening"),
        )
        .author(Author::new("Alice Person").email("alice@example.com"))
        .author(Author::new("Bob Human"))
        .render();

    assert_snapshot!(page);
}

#[test]
fn manual_snapshot_covers_minimal_output() {
    assert_snapshot!(Manual::new("demo").render());
}
