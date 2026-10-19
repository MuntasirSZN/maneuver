# maneuver
[![crates.io version][1]][2] [![downloads][3]][2] [![docs.rs docs][4]][5]

Generate structured man pages using
[roff-rs](https://github.com/killercup/roff-rs).

- [Documentation][5]
- [Crates.io][2]

---

This crate was originally in https://github.com/rust-cli/man, but as there no maintenance for
a long time, it was forked and is now maintained here.

## Usage
```rust
use maneuver::prelude::*;

fn main() {
    let page = Manual::new("basic")
        .about("A basic example")
        .version("1.2.3")
        .source("basic-tools")
        .author(Author::new("Alice Person").email("alice@person.com"))
        .author(Author::new("Bob Human").email("bob@human.com"))
        .flag(
            Flag::new()
                .short("-d")
                .long("--debug")
                .help("Enable debug mode"),
        )
        .flag(
            Flag::new()
                .short("-v")
                .long("--verbose")
                .help("Enable verbose mode"),
        )
        .option(
            Opt::new("output")
                .short("-o")
                .long("--output")
                .help("The file path to write output to"),
        )
        .example(
            Example::new()
                .text("run basic in debug mode")
                .command("basic -d")
                .output("Debug Mode: basic will print errors to the console")
            )
        .custom(
            Section::new("usage note")
                .paragraph("This program will overwrite any file currently stored at the output path")
        )
        .render();

    println!("{}", page);
}
```

`Manual::render` returns a complete roff document. `Manual::path()` returns the
conventional `<name>.1` filename, which is useful when writing pages during a
build:

```rust,no_run
// build.rs
use maneuver::prelude::*;
use std::{env, fs};

fn main() {
    let manual = Manual::new("basic")
        .about("A basic example")
        .source("basic-tools");
    let filename = manual.path();
    let page = manual.render();
    let path = env::var_os("OUT_DIR").unwrap().to_owned();
    let path = std::path::PathBuf::from(path).join(filename);
    fs::write(path, page).unwrap();
}
```

Install the generated `<name>.1` file into the platform's man-directory as
part of your package or Makefile install step. The crate only generates roff;
it does not install files or require a particular build system.

## Metadata and flexible sections

The `version`, `date`, and `source` methods populate the corresponding man
header metadata. No version is set by default, because the manual page
describes the program that uses this crate rather than the crate itself; call
`version("1.2.3")` to fill the field in and `without_version()` to leave it
empty on purpose. The `maneuver::crate_version!` and `maneuver::crate_name!`
macros expand at the call site, so they report the version and the name of the
program being documented:

```rust
use maneuver::prelude::*;

Manual::new(maneuver::crate_name!())
    .version(maneuver::crate_version!())
    .render();
```

Add application-specific exit codes with
`status(ExitStatus::new("CODE", "description"))`, and use
`Opt::separator(" ")` for option forms such as `--output FILE`.
Call `without_exit_status()` to omit the section entirely.

Authors can contain additional text with repeated `Author::text` calls.
Positional arguments support `Arg::optional()` and `Arg::variadic()`, and
`Manual::arg` accepts either one `Arg` or a `Vec<Arg>`. Arguments with a
`help` text are listed in an `ARGUMENTS` section. Use `Section` for
commands, subcommands, SEE ALSO, FILES, or any other project specific section:

```rust
use maneuver::prelude::*;

Manual::new("git-like")
    .custom(
        Section::new("COMMANDS")
            .paragraph("add      Add files to the index.")
            .paragraph("status   Show the working tree status."),
    )
    .render();
```

## Subcommands

`Command` documents a subcommand. Its synopsis is derived from the command
name and the flags, options and arguments it accepts, so it never drifts from
the entries listed underneath it. A manual that has at least one command
mentions `[COMMAND]` in its synopsis:

```rust
use maneuver::prelude::*;

Manual::new("git-like")
    .command(
        Command::new("add")
            .help("Add file contents to the index.")
            .flag(Flag::new().long("--all").help("Add all changed files."))
            .argument(Arg::new("PATH").help("The file to add.")),
    )
    .render();
```

Renders as:

<pre>
<b>SYNOPSIS</b>
       <b>git-like</b> [COMMAND]

<b>COMMANDS</b>
       <b>add</b> [FLAGS] PATH
              Add file contents to the index.

              <b>--all</b>
                     Add all changed files.

              <i>PATH</i>
                     The file to add.
</pre>

Preview by running:
```sh
$ cargo run > /tmp/app.man; man /tmp/app.man
```
Which outputs:
<pre>
BASIC(1)                       General Commands Manual                          BASIC(1)

<b>NAME</b>
       basic - A basic example

<b>SYNOPSIS</b>
       <b>basic</b> [FLAGS] [OPTIONS]

<b>FLAGS</b>
       <b>-d, --debug</b>
              Enable debug mode

       <b>-v, --verbose</b>
              Enable verbose mode

<b>OPTIONS</b>
       <b>-o, --output</b>=<i>output</i>
              The file path to write output to

<b>USAGE NOTE</b>
       This file will overwrite any file currently stored at the output path.

<b>EXIT STATUS</b>
       <strong>0</strong>      Successful program execution.

       <b>1</b>      Unsuccessful program execution.

       <b>101</b>    The program panicked.

<b>EXAMPLES</b>
       run basic in debug mode
              <b>$ basic -d</b>
              Debug Mode: basic will print errors to the console

<b>AUTHORS</b>
         Alice Person &lt;alice@person.com&gt;
         Bob Human &lt;bob@human.com&gt;

                                                                                BASIC(1)
</pre>

## Installation
If using [cargo-edit](https://github.com/killercup/cargo-edit), install with
```sh
$ cargo add maneuver
```
Otherwise, install by adding to Cargo.toml file's dependency section.

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/maneuver.svg?style=flat-square
[2]: https://crates.io/crates/maneuver
[3]: https://img.shields.io/crates/d/maneuver.svg?style=flat-square
[4]: https://docs.rs/maneuver/badge.svg
[5]: https://docs.rs/maneuver
