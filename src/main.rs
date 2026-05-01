use std::path::PathBuf;
use tracing::info;

mod scanner;
mod backup;
mod analyzer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Logging config
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let path: PathBuf = PathBuf::from("/home/ovsyannikovls-laptop/Downloads");

    // let analyzer = analyzer::TableAnalyzer::new();

    // let analysis = analyzer.analyze(&path)?;

    // println!("Result: {:?}", analysis);

    // backup::backup(&path)?;

    let mut scanner = scanner::Scanner::new();

    scanner.scan(&path)?;

    let result = scanner.result();

    info!("{:#?}", result);

    Ok(())
}