use std::path::PathBuf;

#[derive(Debug)]
pub struct Project {
    path: PathBuf,
}

impl Project {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
        }
    }
}
