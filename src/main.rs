use std::process::ExitCode;

use anstream::{eprintln, println};
use anstyle::{AnsiColor, Color, Style};
use anyhow::Context;

mod args;
mod macros;
mod migrations;
mod primitives;
mod traits;
mod untyped_crate;
mod version;

/// The main entrypoint with a custom error handler.
///
/// For the program logic, see [`migrate_rustdoc_types()`].
fn main() -> ExitCode {
    match migrate_rustdoc_types() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let style = Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::BrightRed)));

            eprintln!("(._.;) {style}Error{style:#}: {error:?}");

            ExitCode::FAILURE
        }
    }
}

/// The main program logic.
fn migrate_rustdoc_types() -> anyhow::Result<()> {
    let args = args::parse_args()?;

    let input = std::fs::read_to_string(&args.input)
        .with_context(|| format!("could not read `--input` file: {}", args.input.display()))?;

    let output = self::migrations::migrate_up(&input, args.to_version)?;

    {
        let blue = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Blue)));

        eprintln!("{blue}Done!{blue:#} :D");
    }

    println!("{output}");

    Ok(())
}
