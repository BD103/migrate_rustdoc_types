use std::process::Command;

use crate::discovery::Test;

pub fn generate_rustdoc_json(test: &Test) -> anyhow::Result<()> {
    let test_folder = test.path.parent().unwrap();

    let status = Command::new("rustup")
        .arg("run")
        .arg(crate::toolchains::get_toolchain(test.format_version))
        .arg("rustdoc")
        .arg(format!("--out-dir={}", test_folder.display()))
        .arg("-Zunstable-options")
        .arg("--output-format=json")
        .arg(&test.path)
        .status()?;

    anyhow::ensure!(status.success());

    Ok(())
}
