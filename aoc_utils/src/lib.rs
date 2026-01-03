use std::path::PathBuf;

pub fn read_input(relative_path_from_crate: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative_path_from_crate);

    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read input file {:?}: {}", path, e))
}
