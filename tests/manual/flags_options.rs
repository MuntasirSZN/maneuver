use insta::assert_snapshot;
use man::{Env, Flag, Manual, Opt};

#[test]
fn manual_renders_flags_with_all_field_combinations() {
    assert_snapshot!(Manual::new("demo")
        .flag(Flag::new().short("-s").long("--short").help("both names"))
        .flag(Flag::new().short("-s"))
        .flag(Flag::new().long("--long"))
        .flag(Flag::new().help("no names"))
        .without_version()
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
        .without_version()
        .render());
}

#[test]
fn manual_renders_options_without_forced_equals() {
    assert_snapshot!(Manual::new("demo")
        .option(
            Opt::new("output")
                .long("--output")
                .separator(" ")
                .help("where to write"),
        )
        .without_version()
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
        .without_version()
        .render());
}
