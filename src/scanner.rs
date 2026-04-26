use std::fs;
use std::io;
use std::path::Path;
use tracing::info;
use std::collections::HashMap;

pub struct Scanner {
    extensions: HashMap<String, usize>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            extensions: HashMap::new(),
        }
    }

    pub fn scan(&mut self, dir: &Path) -> io::Result<()> {
        info!("Scanning: {:?}...", dir);
        
        let entries: fs::ReadDir = fs::read_dir(dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                self.scan(&path)?;
            } else if path.is_file() {
                self.add_file(&path);
            }
        }

        Ok(())
    }

    fn add_file(&mut self, path: &Path) {
        let ext = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        let counter: &mut usize = self.extensions.entry(ext).or_insert(0);
        *counter += 1;
    }

    pub fn result(self) -> HashMap<String, usize> {
        self.extensions
    }
}