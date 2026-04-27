use std::fs::File;
use std::path::{Path, PathBuf};
use polars::prelude::*;
use calamine::{open_workbook_auto, Reader, Range};

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
    pub missings: MissingReport,
}

#[derive(Debug)]
pub struct MissingReport {
    pub by_column: Vec<ColumnMissing>,
    pub by_row: Vec<RowMissing>
} 

#[derive(Debug)]
pub struct ColumnMissing {
    pub name: String,
    pub count: usize,
    pub percent: f64,
}

#[derive(Debug)]
pub struct RowMissing {
    pub name: usize,
    pub count: usize,
    pub percent: f64,
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

                Ok(Self::analysis_polars_from_df(path, TableFormat::Csv, df))
            }
            TableFormat::Json => {
                let file: File = File::open(path)?;
                let df: DataFrame = JsonReader::new(file)
                    .finish()?;

                Ok(Self::analysis_polars_from_df(path, TableFormat::Json, df))
            }
            TableFormat::Parquet => {
                let file = File::open(path)?;
                let df: DataFrame = ParquetReader::new(file)
                    .finish()?;

                Ok(Self::analysis_polars_from_df(path, TableFormat::Parquet, df))
            }
            TableFormat::Excel => {
                let mut workbook = open_workbook_auto(path)?;

                let sheet_name: String = workbook
                    .sheet_names()
                    .first()
                    .cloned()
                    .ok_or("Workbook has no sheets")?;

                let range: calamine::Range<calamine::Data> = workbook.worksheet_range(&sheet_name)?;

                Ok(Self::analysis_calamine_from_df(path, TableFormat::Excel, range))
            }
            TableFormat::Unsupported => {
                let missings: MissingReport = MissingReport {
                    by_column: Vec::new(),
                    by_row: Vec::new(),
                };
                Ok(TableAnalysis {
                    path: path.to_path_buf(),
                    format: TableFormat::Unsupported,
                    rows: 0,
                    columns: 0,
                    column_names: Vec::new(),
                    missings: missings,
                })
            }
        }
    }

    fn analysis_polars_from_df(
        path: &Path,
        format: TableFormat,
        df: DataFrame
    ) -> TableAnalysis {

        let column_names: Vec<String> = df
            .get_column_names()
            .iter()
            .map(|name| name.to_string())
            .collect();

        let missings: MissingReport = Self::missings_from_polars(df);

        TableAnalysis {
            path: path.to_path_buf(),
            format: format,
            rows: df.height(),
            columns: df.width(),
            column_names: column_names,
            missings: missings,
        }
    }

    fn analysis_calamine_from_df(
        path: &Path,
        format: TableFormat,
        range: calamine::Range<calamine::Data>,
    ) -> TableAnalysis {

        let column_names: Vec<String> = range
            .rows()
            .next()
            .unwrap_or(&[])
            .iter()
            .map(|cell| cell.to_string())
            .collect();

        let missings: calamine::Range<calamine::Data> = Self::missings_from_calamine(range);

        TableAnalysis {
            path: path.to_path_buf(),
            format: format,
            rows: range.height(),
            columns: range.width(),
            column_names: column_names,
            missings: missings,
        }
    }

    fn missings_from_polars(df: DataFrame) {
        let total_rows: usize = df.height();
        let total_columns: usize = df.width();


    }

    fn missings_from_calamine(range: calamine::Range<calamine::Data>) {
        let total_rows: usize = range.height();
        let total_columns: usize = range.width();


    }
}