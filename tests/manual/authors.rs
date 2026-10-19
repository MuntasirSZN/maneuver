use insta::assert_snapshot;
use maneuver::{Author, Manual};

#[test]
fn manual_renders_single_and_multiple_authors() {
    assert_snapshot!(
        "single_author",
        Manual::new("demo")
            .author(Author::new("Alice").email("alice@example.com"))
            .without_version()
            .render()
    );

    assert_snapshot!(
        "multiple_authors",
        Manual::new("demo")
            .author(Author::new("Alice"))
            .author(Author::new("Bob").email("bob@example.com"))
            .without_version()
            .render()
    );

    assert_snapshot!(
        Manual::new("demo")
            .author(
                Author::new("Project Team")
                    .email("team@example.com")
                    .text("Maintained by the platform group.")
                    .text("Contact us for contribution questions.")
            )
            .without_version()
            .render()
    );
}
