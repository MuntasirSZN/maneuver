use insta::assert_snapshot;
use maneuver::{Arg, Manual};

#[test]
fn empty_manual_contains_required_sections() {
    assert_snapshot!(Manual::new("demo").without_version().render());
}

#[test]
fn manual_renders_description_and_positional_arguments() {
    assert_snapshot!(
        Manual::new("demo")
            .about("short description")
            .description("long description")
            .arg(Arg::new("input"))
            .arg(Arg::new("output"))
            .without_version()
            .render()
    );
}

#[test]
fn manual_renders_optional_and_variadic_arguments() {
    assert_snapshot!(
        Manual::new("demo")
            .arg(Arg::new("config").optional())
            .arg(Arg::new("files").variadic())
            .without_version()
            .render()
    );
}

#[test]
fn manual_accepts_argument_vectors() {
    assert_snapshot!(
        Manual::new("demo")
            .arg(vec![
                Arg::new("config").optional(),
                Arg::new("files").variadic(),
            ])
            .without_version()
            .render()
    );
}
