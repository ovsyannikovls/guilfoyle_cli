use std::fs::File;
use std::path::{Path, PathBuf};
use polars::prelude::*;
use calamine::{open_workbook_auto, Reader};

#[derive(Debug)]
pub enum TableFormat {
    Csv,
    Json,
    Parquet,
    Excel,
    Unsupported,
}

#[derive(Debug)]
pub struct TableAnalysis {
    pub path: PathBuf,
    pub format: TableFormat,
    pub rows: usize,
    pub columns: usize,
    pub column_names: Vec<String>,
}

pub struct TableAnalyzer;

impl TableAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, path: &Path) -> Result<TableAnalysis, Box<dyn std::error::Error>> {
        match Self::detect_table_format(path) {
            TableFormat::Csv => Self::format_analyze(path, TableFormat::Csv),
            TableFormat::Json => Self::format_analyze(path, TableFormat::Json),
            TableFormat::Parquet => Self::format_analyze(path, TableFormat::Parquet),
            TableFormat::Excel => Self::format_analyze(path, TableFormat::Excel),
            TableFormat::Unsupported => Self::format_analyze(path, TableFormat::Unsupported),        
        }
    }

    fn detect_table_format(path: &Path) -> TableFormat {
        match path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase()
            .as_str()
        {
            "csv" => TableFormat::Csv,
            "json" => TableFormat::Json,
            "parquet" => TableFormat::Parquet,
            "xlsx" | "xls" | "xlsm" => TableFormat::Excel,
            _ => TableFormat::Unsupported,
        }
    }

    fn format_analyze(path: &Path, format: TableFormat) -> Result<TableAnalysis, Box<dyn std::error::Error>> {
        match format {
            TableFormat::Csv => {
                let df: DataFrame = CsvReadOptions::default()
                    .with_has_header(true)
                    .try_into_reader_with_file_path(Some(path.into()))?
                    .finish()?;

                Ok(Self::analysis_from_df(path, TableFormat::Csv, df))
            }
            TableFormat::Json => {
                let file: File = File::open(path)?;
                let df: DataFrame = JsonReader::new(file)
                    .finish()?;

                Ok(Self::analysis_from_df(path, TableFormat::Json, df))
            }
            TableFormat::Parquet => {
                let file = File::open(path)?;
                let df: DataFrame = ParquetReader::new(file)
                    .finish()?;

                Ok(Self::analysis_from_df(path, TableFormat::Parquet, df))
            }
            TableFormat::Excel => {
                let mut workbook = open_workbook_auto(path)?;

                let sheet_name: String = workbook
                    .sheet_names()
                    .first()
                    .cloned()
                    .ok_or("Workbook has no sheets")?;

                let range = workbook.worksheet_range(&sheet_name)?;

                let column_names: Vec<String> = range
                    .rows()
                    .next()
                    .unwrap_or(&[])
                    .iter()
                    .map(|cell| cell.to_string())
                    .collect();

                Ok(TableAnalysis {
                    path: path.to_path_buf(),
                    format: TableFormat::Excel,
                    rows: range.height(),
                    columns: range.width(),
                    column_names: column_names,
                })
            }
            TableFormat::Unsupported => {
                Ok(TableAnalysis {
                    path: path.to_path_buf(),
                    format: TableFormat::Unsupported,
                    rows: 0,
                    columns: 0,
                    column_names: Vec::new(),
                })
            }
        }
    }

    fn analysis_from_df(
        path: &Path,
        format: TableFormat,
        df: DataFrame
    ) -> TableAnalysis {

        let column_names: Vec<String> = df
            .get_column_names()
            .iter()
            .map(|name| name.to_string())
            .collect();

        TableAnalysis {
            path: path.to_path_buf(),
            format: format,
            rows: df.height(),
            columns: df.width(),
            column_names,
        }
    }
}