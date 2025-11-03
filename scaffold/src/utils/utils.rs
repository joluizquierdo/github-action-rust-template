use std::path::Path;

pub fn join_path(base_path: &str, relative_path: &str) -> String {
    let err_msg = &format!(
        "Failed to join repository root path '{}' with relative path '{}'",
        base_path, relative_path
    );

    Path::new(&base_path)
        .join(relative_path)
        .to_str()
        .expect(err_msg)
        .to_string()
}

pub fn remove_no_longer_required_files(files: &[String]) {
    for file in files {
        let path = Path::new(file);
        if path.exists() {
            println!("Removing no longer required file: {}", file);
        } else {
            println!("File not found, skipping removal: {}", file);
            continue;
        }

        let err_msg = &format!("Failed to remove file or directory: {}", file);
        if path.is_dir() {
            std::fs::remove_dir_all(path).expect(err_msg);
        } else {
            std::fs::remove_file(path).expect(err_msg);
        }
    }
}
