use insta::assert_snapshot;
use maneuver::{Arg, Command, Flag, Manual, Opt};

#[test]
fn manual_renders_commands_with_synopsis_and_description() {
    assert_snapshot!(
        Manual::new("demo")
            .command(Command::new("check").help("Check availability for a file."))
            .command(Command::new("clean").help("Remove artifacts."))
            .without_version()
            .render()
    );
}

#[test]
fn manual_renders_command_options_flags_and_arguments() {
    assert_snapshot!(
        Manual::new("demo")
            .command(
                Command::new("show")
                    .help("Show a variable in a function.")
                    .flag(Flag::new().long("--all-targets").help("Check all targets."))
                    .option(
                        Opt::new("PATH")
                            .long("--path")
                            .separator(" ")
                            .help("The file to analyze."),
                    )
                    .argument(Arg::new("FUNCTION_PATH").help("The function to analyze.")),
            )
            .without_version()
            .render()
    );
}

#[test]
fn manual_renders_command_subcommand_as_positional_argument() {
    assert_snapshot!(
        Manual::new("demo")
            .command(
                Command::new("toolchain")
                    .help("Install or uninstall the toolchain.")
                    .argument(Arg::new("COMMAND").optional()),
            )
            .without_version()
            .render()
    );
}

#[test]
fn manual_synopsis_mentions_command_when_commands_exist() {
    let page = Manual::new("demo")
        .command(Command::new("check"))
        .without_version()
        .render();
    assert!(page.contains("[COMMAND]"));
}
