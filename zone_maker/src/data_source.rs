use serde_json::Value as Json;

///
pub trait DataSource {
    ///
    fn get_row(&self, key: &str) -> Option<&serde_json::Map<String, Json>>;

    ///
    fn get_all_rows(&self) -> Vec<&serde_json::Map<String, Json>>;
}

///
pub mod excel_data_source;
