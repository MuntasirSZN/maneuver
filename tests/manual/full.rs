use insta::assert_snapshot;
use man::{Arg, Author, Env, Example, Flag, Manual, Opt, Section};

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
        .without_version()
        .render();

    assert_snapshot!(page);
}

#[test]
fn manual_snapshot_covers_minimal_output() {
    assert_snapshot!(Manual::new("demo").without_version().render());
}
