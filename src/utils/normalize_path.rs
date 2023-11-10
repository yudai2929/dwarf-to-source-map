pub fn normalize_path(path: &str) -> String {
    let path = path.replace("\\", "/");
    let path = path.replace("./", "");
    let path = path.replace("../", "");
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_path() {
        assert_eq!(
            normalize_path(r"C:\Users\user\.\..\file.rs"),
            "C:/Users/user/file.rs"
        );
        assert_eq!(normalize_path(r"./../file.rs"), "file.rs");
        assert_eq!(normalize_path(r"file.rs"), "file.rs");
    }
}
