mod discovery;
mod generate;
mod toolchains;

fn main() -> anyhow::Result<()> {
    let tests = dbg!(self::discovery::discover_tests()?);

    for test in tests {
        self::toolchains::install_toolchain(test.format_version)?;

        self::generate::generate_rustdoc_json(&test)?;

        // self::toolchains::uninstall_toolchain(test.format_version)?;
    }

    Ok(())
}
