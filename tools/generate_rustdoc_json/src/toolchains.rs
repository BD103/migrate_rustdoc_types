use std::process::Command;

/// An indexed lookup table that associates a Rustup toolchain with a specific `rustdoc` format
/// version.
///
/// For example, `TOOLCHAINS[0]` returns the Rustup toolchain name that generates `rustdoc` JSON
/// with the v1 format version.
static TOOLCHAINS: [&str; 45] = [
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "nightly-2025-02-24", // v40
    "nightly-2025-03-10",
    "nightly-2025-03-15",
    "nightly-2025-03-22",
    "nightly-2025-04-15",
    "nightly-2025-04-18", // v45
];

/// Returns the name of the Rustup toolchain that will generate `rustdoc` JSON for a given format
/// version.
pub fn get_toolchain(format_version: u32) -> &'static str {
    let toolchain = TOOLCHAINS[format_version as usize - 1];

    assert!(
        !toolchain.is_empty(),
        "format version {format_version} is not yet supported"
    );

    toolchain
}

/// Installs the Rustup toolchain necessary to generate a given `rustdoc` JSON format version.
pub fn install_toolchain(format_version: u32) -> anyhow::Result<()> {
    let status = Command::new("rustup")
        .arg("toolchain")
        .arg("install")
        .arg("--profile=minimal")
        .arg(get_toolchain(format_version))
        .status()?;

    anyhow::ensure!(status.success());

    Ok(())
}

/// Uninstalls the Rustup toolchain used to generate a given `rustdoc` JSON format version.
pub fn uninstall_toolchain(format_version: u32) -> anyhow::Result<()> {
    let status = Command::new("rustup")
        .arg("toolchain")
        .arg("uninstall")
        .arg(get_toolchain(format_version))
        .status()?;

    anyhow::ensure!(status.success());

    Ok(())
}
