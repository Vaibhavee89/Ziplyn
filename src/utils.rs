use std::path::Path;

/// Validates if a file exists.
pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

/// Appends a `.gz` extension to a file path if missing.
pub fn ensure_gz_extension(path: &str) -> String {
    if !path.ends_with(".gz") {
        format!("{}.gz", path)
    } else {
        path.to_string()
}
}
