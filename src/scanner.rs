use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;
use tracing::info;
use std::collections::HashMap;

/*
Переработать так чтобы к каждому файлу подписывалось валиден ли он и была проверка на это

Идеальный пайплайн (Продумать как можно улучшить каждый вариант):

1. Ответ, доступна ли вообще директория
2. Список всех форматов файлов с их количеством
3. Список всех невалидных файлов с их путями
...

*/

pub struct Scanner {
    extensions: HashMap<String, usize>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            extensions: HashMap::new(),
        }
    }

    pub fn scan(&mut self, path: &Path) -> io::Result<()> {
        info!("Scanning: {:?}...", path);

        let entries: fs::ReadDir = fs::read_dir(path)?;

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
        let ext = match Self::is_valid(path) {
            true => {
                path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("")
                    .to_lowercase()
            },
            false => format!("unreadable: {:#?}", path).to_string(),
        };


        let counter: &mut usize = self.extensions.entry(ext).or_insert(0);
        *counter += 1;
    }

    fn is_valid(path: &Path) -> bool {
        File::open(path).is_ok()
    }

    fn sort_result(&mut self) ->  Vec<(String, usize)> {
        let mut items: Vec<String, usize> = self.extensions.into_iter().collect();

        items
    }

    pub fn result(self) -> Vec<(String, usize)> {
        self.sort_result()

        // self.extensions
    }
}