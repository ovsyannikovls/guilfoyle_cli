use std::fs;
use std::io;
use tracing::{debug, error, info, warn};

// Scan imports 
use std::collections::HashMap;

fn main() -> io::Result<()>{
    // Logging config
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();


    let scan_result: HashMap<String, usize> = scan("/home/ovsyannikovls/Desktop/VScode projects/guilfoyle")?;
    info!("{:#?}", scan_result);

    Ok(())
}

fn scan(dir: &str) -> io::Result<HashMap<String, usize>>{
    info!("Scanning starts...");

    let mut hash_map: HashMap<String, usize> = HashMap::new();

    recursive_search(dir, hash_map);

    Ok(hash_map)
}

fn recursive_search(dir: PathBuf, hash_map: &mut HashMap<String, usize>) {
    let dir: fs::ReadDir = fs::read_dir(dir)?;

    for el in dir{
        let el = el?;
        
        if el.is_dir() {
            recursive_search(el, hash_map)
        } else {
            add_ext_to_hash_map(el, hash_map);
        }
    }
}

fn add_ext_to_hash_map(file: &str, hash_map: &mut HashMap<String, usize>) {
    let ext: &str = get_file_extension(file);
    let ext_result: io::Result<i64> = hash_map.get(ext);

    match ext_result {
        !None => hash_map.insert(ext, ext_result + 1),
        None => hash_map.insert(ext, 1),
    }
}

fn get_file_extension(file_name: &str) -> &str{
    let extension: &str = file_name.split(".").last().unwrap_or("None");

    return extension;
}