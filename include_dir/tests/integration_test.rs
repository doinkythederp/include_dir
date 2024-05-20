use include_dir::{include_dir, Dir};

static PARENT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR");

#[test]
fn included_all_files_in_the_include_dir_crate() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    validate_included(&PARENT_DIR, root, root);
    assert!(PARENT_DIR.contains("src/lib.rs"));
}

#[cfg(feature = "std")]
#[test]
fn extract_all_files() {
    use tempfile::TempDir;
    let tmpdir = TempDir::new().unwrap();
    let root = tmpdir.path();
    PARENT_DIR.extract(root).unwrap();

    validate_extracted(&PARENT_DIR, root);
}

// Validates that all files on the filesystem exist in the inclusion
fn validate_included(dir: &Dir<'_>, path: &std::path::Path, root: &std::path::Path) {
    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap().path();
        let entry = entry.strip_prefix(root).unwrap();

        let name = entry.file_name().unwrap();

        assert!(
            dir.contains(entry.to_str().unwrap()),
            "Can't find {}",
            entry.display()
        );

        if entry.is_dir() {
            let child_path = path.join(name);
            validate_included(
                dir.get_entry(entry.to_str().unwrap())
                    .unwrap()
                    .as_dir()
                    .unwrap(),
                &child_path,
                root,
            );
        }
    }
}

// Validates that all files in the inclusion were extracted to the filesystem
#[cfg(feature = "std")]
fn validate_extracted(dir: &Dir, path: &std::path::Path) {
    // Check if all the subdirectories exist, recursing on each
    for subdir in dir.dirs() {
        let subdir_path = path.join(dir.path().to_str().unwrap());
        assert!(subdir_path.exists());
        validate_extracted(subdir, &subdir_path);
    }

    // Check if the files at the root of this directory exist
    for file in dir.files() {
        let file_path = path.join(file.path().to_str().unwrap());
        assert!(file_path.exists());
    }
}
