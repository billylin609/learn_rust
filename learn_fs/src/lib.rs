//extra note: the home position is relative to the cargo.toml file
pub fn get_absolute_path(relative_path: &str) {
    println!("The path input is: {}", relative_path);
    let path: String = match std::fs::canonicalize(relative_path) {
        Ok(p) => p.to_str().unwrap().to_string(),
        Err(_) => {
            panic!("error find");
        },
    };
    println!("The absoulte path for the file is：{}", path);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonicalize() {
        get_absolute_path("test_read_from/test.txt");
    }
}
