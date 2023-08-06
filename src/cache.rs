use std::path::PathBuf;

struct Cache {
    root_dir: PathBuf,
}

impl Cache {
    fn new(root_dir: PathBuf) -> Self {
        Self {
            root_dir,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache() {
        let cache = Cache::new(PathBuf::from("/tmp"));
    }
}