use std::{
    ffi::OsStr,
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Test {
    pub path: PathBuf,
    pub format_version: u32,
}

pub fn discover_tests() -> io::Result<Vec<Test>> {
    let mut tests = Vec::new();

    let test_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests");

    for dir_entry in fs::read_dir(test_dir)? {
        let dir_entry = dir_entry?;

        if dir_entry.file_type()?.is_dir() {
            let Some(format_version) = extract_format_version(&dir_entry) else {
                continue;
            };

            for test_entry in fs::read_dir(dir_entry.path())? {
                let test_path = test_entry?.path();

                if test_path.extension() == Some(OsStr::new("rs")) {
                    tests.push(Test {
                        path: test_path,
                        format_version,
                    });
                }
            }
        }
    }

    Ok(tests)
}

fn extract_format_version(entry: &DirEntry) -> Option<u32> {
    let name = entry.file_name();

    name.to_string_lossy()
        .strip_prefix("v")
        .and_then(|s| s.parse().ok())
}
