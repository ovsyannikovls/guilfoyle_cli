use std::fs;
use std::io;
use chrono::Local;
use std::path::{Path, PathBuf};
use tracing::info;


fn copy_dir_all(from_dir: &Path, to_dir: &Path) -> io::Result<()> {
    fs::create_dir_all(to_dir)?;

    for entry in fs::read_dir(from_dir)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        
        let from = entry.path();
        let to = to_dir.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_all(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
        }
    }

    Ok(())
}

pub fn backup(dir: &Path) -> io::Result<()> {
    let now = Local::now().format("%Y-%m-%d_%H-%M-%S");

    let backup_dir: PathBuf = dir.with_file_name(format!(
        "{}_BACKUP_{}",
        dir.file_name()
            .unwrap()
            .to_string_lossy(),
        now
    ));

    copy_dir_all(dir, &backup_dir)?;

    info!("BackUp - {}", &backup_dir.display());

    Ok(())
}