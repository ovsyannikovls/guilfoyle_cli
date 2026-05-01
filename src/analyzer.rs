// use std::fs::File;
// use std::path::{Path, PathBuf};
// use polars::prelude::*;
// use calamine::{open_workbook_auto, Reader, Data};

// /*
// Сделать так, чтобы анализировались строки и выдавались типы данных, на которые они похожи

// Идеальный пайплайн (Продумать как можно улучшить каждый вариант):

// 1. Посмотреть первые пять строк с сжатыми столбцами (как .head() в pandas)
// 2. Посмотреть кол-во колонок и строк, а также названия всех колонок
// 3. Посмотреть типы данных в каждой колонке (номинальные / реальные)
// 4. Посмотреть пропуски по колонкам и строкам (по колонкам: абсолютное и в процентах, по строкам: стреднее в проценах)
// ...

// */

// #[derive(Debug)]
// pub enum TableFormat {
//     Csv,
//     Json,
//     Parquet,
//     Excel,
//     Unsupported,
// }

// pub enum TypeLike {
//     integer_like,
//     float_like,
//     bool_like,
//     date_like,
//     string_like,
//     missing,
// }

// #[derive(Debug)]
// pub struct TableAnalysis {
//     pub path: PathBuf,
//     pub format: TableFormat,
//     pub rows: usize,
//     pub columns: usize,
//     pub column_names: Vec<String>,
//     pub missings: MissingReport,
//     pub columns_types: Vec<ColumnTypes>,
// }

// #[derive(Debug)]
// pub struct ColumnTypes {
//     pub name: String,
//     pub polars_dtype: String,
//     pub integer_like: usize,
//     pub float_like: usize,
//     pub bool_like: usize,
//     pub date_like: usize,
//     pub string_like: usize,
//     pub missing: usize,
// } 

// #[derive(Debug)]
// pub struct MissingReport {
//     pub by_columns: Vec<ColumnMissing>,
//     pub by_rows: Vec<RowMissing>
// } 

// #[derive(Debug)]
// pub struct ColumnMissing {
//     pub name: String,
//     pub count: usize,
//     pub percent: f64,
// }

// #[derive(Debug)]
// pub struct RowMissing {
//     pub index: usize,
//     pub count: usize,
//     pub percent: f64,
// }

// pub struct TableAnalyzer;

// impl TableAnalyzer {
//     pub fn new() -> Self {
//         Self
//     }

//     pub fn analyze(&self, path: &Path) -> Result<TableAnalysis, Box<dyn std::error::Error>> {
//         match Self::detect_table_format(path) {
//             TableFormat::Csv => Self::format_analyze(path, TableFormat::Csv),
//             TableFormat::Json => Self::format_analyze(path, TableFormat::Json),
//             TableFormat::Parquet => Self::format_analyze(path, TableFormat::Parquet),
//             TableFormat::Excel => Self::format_analyze(path, TableFormat::Excel),
//             TableFormat::Unsupported => Self::format_analyze(path, TableFormat::Unsupported),        
//         }
//     }

//     fn detect_table_format(path: &Path) -> TableFormat {
//         match path
//             .extension()
//             .and_then(|ext| ext.to_str())
//             .unwrap_or("")
//             .to_lowercase()
//             .as_str()
//         {
//             "csv" => TableFormat::Csv,
//             "json" => TableFormat::Json,
//             "parquet" => TableFormat::Parquet,
//             "xlsx" | "xls" | "xlsm" => TableFormat::Excel,
//             _ => TableFormat::Unsupported,
//         }
//     }

//     fn format_analyze(path: &Path, format: TableFormat) -> Result<TableAnalysis, Box<dyn std::error::Error>> {
//         match format {
//             TableFormat::Csv => {
//                 let df: DataFrame = CsvReadOptions::default()
//                     .with_has_header(true)
//                     .try_into_reader_with_file_path(Some(path.into()))?
//                     .finish()?;

//                 Ok(Self::analysis_polars_from_df(path, TableFormat::Csv, df))
//             }
//             TableFormat::Json => {
//                 let file: File = File::open(path)?;
//                 let df: DataFrame = JsonReader::new(file)
//                     .finish()?;

//                 Ok(Self::analysis_polars_from_df(path, TableFormat::Json, df))
//             }
//             TableFormat::Parquet => {
//                 let file: File = File::open(path)?;
//                 let df: DataFrame = ParquetReader::new(file)
//                     .finish()?;

//                 Ok(Self::analysis_polars_from_df(path, TableFormat::Parquet, df))
//             }
//             TableFormat::Excel => {
//                 let mut workbook = open_workbook_auto(path)?;

//                 let sheet_name: String = workbook
//                     .sheet_names()
//                     .first()
//                     .cloned()
//                     .ok_or_else(|| {
//                         std::io::Error::new(
//                             std::io::ErrorKind::InvalidData,
//                             "Workbook has no sheets",
//                         )
//                     })?;

//                 let range: calamine::Range<calamine::Data> = workbook.worksheet_range(&sheet_name)?;

//                 Ok(Self::analysis_calamine_from_df(path, TableFormat::Excel, range))
//             }
//             TableFormat::Unsupported => {
//                 let missings: MissingReport = MissingReport {
//                     by_columns: Vec::new(),
//                     by_rows: Vec::new(),
//                 };
//                 Ok(TableAnalysis {
//                     path: path.to_path_buf(),
//                     format: TableFormat::Unsupported,
//                     rows: 0,
//                     columns: 0,
//                     column_names: Vec::new(),
//                     missings: missings,
//                     columns_types: Vec::new(),
//                 })
//             }
//         }
//     }

//     fn analysis_polars_from_df(
//         path: &Path,
//         format: TableFormat,
//         df: DataFrame
//     ) -> TableAnalysis {

//         let column_names: Vec<String> = df
//             .get_column_names()
//             .iter()
//             .map(|name| name.to_string())
//             .collect();

//         let missings: MissingReport = Self::missings_from_polars(&df);

//         let columns_types: Vec<ColumnTypes> = Self::count_polars_column_types(&df)?;

//         TableAnalysis {
//             path: path.to_path_buf(),
//             format: format,
//             rows: df.height(),
//             columns: df.width(),
//             column_names: column_names,
//             missings: missings,
//             columns_types: columns_types,
//         }
//     }

//     fn analysis_calamine_from_df(
//         path: &Path,
//         format: TableFormat,
//         range: calamine::Range<calamine::Data>,
//     ) -> TableAnalysis {

//         let column_names: Vec<String> = range
//             .rows()
//             .next()
//             .unwrap_or(&[])
//             .iter()
//             .map(|cell| cell.to_string())
//             .collect();

//         let missings: MissingReport = Self::missings_from_calamine(&range);

//         let columns_types: Vec<ColumnTypes> = Self::count_polars_column_types(&range)?;

//         TableAnalysis {
//             path: path.to_path_buf(),
//             format: format,
//             rows: range.height(),
//             columns: range.width(),
//             column_names: column_names,
//             missings: missings,
//             columns_types: columns_types,
//         }
//     }

//     fn missings_from_polars(df: &DataFrame) -> MissingReport{
//         let total_rows: usize = df.height();
//         let total_columns: usize = df.width();

//         let by_columns: Vec<ColumnMissing> = df.columns()
//             .iter()
//             .map(|column| {
//                 let count: usize = (0..total_rows)
//                     .filter(|row_index| {
//                         let value = column
//                             .get(*row_index)
//                             .unwrap_or(AnyValue::Null);

//                         Self::is_missing_polars_value(&value)
//                     })
//                     .count();

//                 let percent: f64 = if total_rows == 0 {
//                     0.0
//                 } else {
//                     (count as f64 / total_rows as f64) * 100.0
//                 };

//                 ColumnMissing {
//                     name: column.name().to_string(),
//                     count,
//                     percent,
//                 }
//             })
//             .collect();

//         let mut by_rows: Vec<RowMissing> = Vec::new();

//         for row_index in 0..total_rows {
//             let mut count: usize = 0;

//             for column in df.columns() {
//                 let value = column.get(row_index).unwrap_or(AnyValue::Null);

//             if Self::is_missing_polars_value(&value) {
//                 count += 1;
//             }
//             }

//             let percent: f64 = if total_columns == 0 {
//                 0.0
//             } else {
//                 (count as f64 / total_columns as f64) * 100.0
//             };

//             by_rows.push(RowMissing {
//                 index: row_index,
//                 count: count,
//                 percent: percent,
//             });
//         }

//         MissingReport {
//             by_columns: by_columns,
//             by_rows:  by_rows,
//         }
//     }

//     fn missings_from_calamine(range: &calamine::Range<calamine::Data>) -> MissingReport {
//         let total_rows: usize = range.height();
//         let total_columns: usize = range.width();

//         let column_names: Vec<String> = range
//             .rows()
//             .next()
//             .unwrap_or(&[])
//             .iter()
//             .map(|cell| cell.to_string())
//             .collect();

//         let mut column_counts: Vec<usize> = vec![0; total_columns];

//         for row in range.rows() {
//             for column_index in 0..total_columns {
//                 let is_missing = match row.get(column_index) {
//                     Some(cell) => Self::is_missing_calamine_value(cell),
//                     None => true,
//                 };

//                 if is_missing {
//                     column_counts[column_index] += 1;
//                 }
//             }
//         }

//         let by_columns: Vec<ColumnMissing> = column_counts
//             .into_iter()
//             .enumerate()
//             .map(|(index, count)| {
//                 let name: String = column_names.get(index)
//                     .cloned()
//                     .unwrap_or_else(|| format!("column_{}", index));

//                 let percent: f64 = if total_rows == 0 {
//                     0.0
//                 } else {
//                     (count as f64 / total_rows as f64) * 100.0
//                 };

//                 ColumnMissing {
//                     name: name,
//                     count: count,
//                     percent: percent 
//                 }
//             })
//             .collect();

//         let mut by_rows: Vec<RowMissing> = Vec::new();

//         for (row_index, row) in range.rows().enumerate() {
//             let mut count: usize = 0;

//             for column_index in 0..total_columns {
//                 let is_missing = match row.get(column_index) {
//                     Some(cell) => Self::is_missing_calamine_value(cell),
//                     None => true,
//                 };

//                 if is_missing {
//                     count += 1;
//                 }
//             }

//             let percent: f64 = if total_columns == 0 {
//                 0.0
//             } else {
//                 (count as f64 / total_columns as f64) * 100.0
//             };

//             by_rows.push(RowMissing {
//                 index: row_index,
//                 count: count,
//                 percent: percent,
//             });
//         }

//         MissingReport {
//             by_columns: by_columns,
//             by_rows:  by_rows,
//         }
//     }

//     fn is_missing_polars_value(value: &AnyValue) -> bool {
//         match value {
//             AnyValue::Null => true,
//             AnyValue::String(value) => value.trim().is_empty(),
//             AnyValue::StringOwned(value) => value.as_str().trim().is_empty(),
//             _ => false,
//         }
//     }

//     fn is_missing_calamine_value(value: &Data) -> bool {
//         match value {
//             Data::Empty => true,
//             Data::String(s) => s.trim().is_empty(),
//             _ => false,
//         }
//     }

//     fn count_polars_column_types(df: &DataFrame) -> Result<Vec<ColumnTypes>, std::io::Error> {
//         let columns_types: Vec<ColumnTypes> = Vec::new();

//         for column in df.columns() {
//             let integer_like: usize = 0;
//             let float_like: usize = 0;
//             let bool_like: usize = 0;
//             let date_like: usize = 0;
//             let string_like: usize = 0;
//             let missing: usize = 0;

//             columns_types.push(ColumnTypes {
//                 name: column.name(),
//                 polars_dtype: column.dtype(),
//                 integer_like: integer_like,
//                 float_like: float_like,
//                 bool_like: bool_like,
//                 date_like: date_like,
//                 string_like: string_like,
//                 missing: missing,
//             });


//         }

//         Ok(())
//     }

//     fn count_calamine_column_types(range: &calamine::Range<calamine::Data>) -> Result<Vec<ColumnTypes>, std::io::Error> {
        
//         Ok(())
//     }

//     fn polars_type_like_classifier() -> TypeLike {
//         let type_like: TypeLike = TypeLike::integer_like; 



//         type_like
//     }
// }