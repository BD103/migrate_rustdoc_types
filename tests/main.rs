use std::process::Command;

use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};

fn cli() -> Command {
    Command::new(get_cargo_bin("migrate_rustdoc_types"))
}

#[test]
fn migrate() {
    assert_cmd_snapshot!(cli().arg("--input").arg("tests/v44/v44.json").arg("--to-version").arg("45"));
}
