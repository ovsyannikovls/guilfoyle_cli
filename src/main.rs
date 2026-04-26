use std::path::PathBuf;

mod scanner;
mod backup;
mod analyzer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Logging config
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let dir: PathBuf = PathBuf::from("/home/ovsyannikovls/Downloads/monster_com-job_sample.csv");

    let analyzer = analyzer::TableAnalyzer::new();

    let analysis = analyzer.analyze(&dir)?;

    println!("Result: {:#?}", analysis);

    // backup::backup(&dir)?;

    // let mut scanner = Scanner::new();

    // scanner.scan(&dir)?;

    // let result = scanner.result();

    // info!("{:#?}", result);

    Ok(())
}