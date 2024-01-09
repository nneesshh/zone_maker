use std::path::PathBuf;

use serde_json::Value as Json;

use crate::data_source::DataSource;

use self::json_rows::JsonRows;

///
pub mod cell;
pub mod range_helper;
pub mod title;

///
pub mod excel_rows;
pub mod json_rows;

///
pub struct ExcelDataSource {
    json_rows: JsonRows,
}

impl ExcelDataSource {
    ///
    pub fn new(key_name: &str, xlsx_path: &PathBuf) -> Self {
        let json_rows = range_helper::read_json_rows_from_xlsx(key_name, &xlsx_path);
        Self { json_rows }
    }
}

impl DataSource for ExcelDataSource {
    ///
    fn get_row(&self, key: &str) -> Option<&serde_json::Map<String, Json>> {
        let json_row_opt = self.json_rows.get_row_by_key(&key);
        if let Some(json_row) = json_row_opt {
            //
            Some(&json_row.value_table)
        } else {
            None
        }
    }

    ///
    fn get_all_rows(&self) -> Vec<&serde_json::Map<String, Json>> {
        let mut v = Vec::with_capacity(self.json_rows.len());
        for (_row, json_row) in &self.json_rows.row_table {
            //
            v.push(&json_row.value_table);
        }
        v
    }
}
