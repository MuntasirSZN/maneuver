use insta::assert_snapshot;
use maneuver::{Example, Manual, Section};

#[test]
fn manual_renders_custom_sections_and_multiple_paragraphs() {
    assert_snapshot!(
        Manual::new("demo")
            .custom(
                Section::new("usage note")
                    .paragraph("first paragraph")
                    .paragraph("second paragraph"),
            )
            .without_version()
            .render()
    );
}

#[test]
fn manual_renders_examples_with_and_without_output() {
    assert_snapshot!(
        Manual::new("demo")
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
            .without_version()
            .render()
    );
}
