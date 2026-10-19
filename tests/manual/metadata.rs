use insta::assert_snapshot;
use maneuver::{Author, ExitStatus, Manual};

#[test]
fn manual_renders_custom_exit_statuses() {
    assert_snapshot!(
        Manual::new("demo")
            .status(ExitStatus::new("64", "Command line usage error."))
            .status(ExitStatus::new("70", "Internal software error."))
            .without_version()
            .render()
    );
}

#[test]
fn manual_can_omit_exit_statuses() {
    assert_snapshot!(
        Manual::new("demo")
            .without_exit_status()
            .without_version()
            .render()
    );
}

#[test]
fn manual_renders_metadata_and_path() {
    let manual = Manual::new("demo")
        .version("2.4.0")
        .date("2026-10-19")
        .source("demo-tools")
        .author(Author::new("Team").text("Additional author information."));
    assert_eq!(manual.path(), std::path::PathBuf::from("demo.1"));
    assert_snapshot!(manual.render());
}

#[test]
fn manual_uses_package_version_by_default() {
    let page = Manual::new("demo").render();
    assert!(page.contains(env!("CARGO_PKG_VERSION")));
}
