use std::path::PathBuf;

#[derive(Debug)]
pub struct Project {
    path: PathBuf,
    
    name: String,
}

impl Project {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            name: "Testing".into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
