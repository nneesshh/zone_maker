use serde_json::Value as Json;

use crate::utils::json_util;

///
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CellType {
    Unknown,
    Null,
    String,
    Integer,
    Float,
}

///
#[derive(Debug)]
pub struct CellData {
    pub row: u32, // row
    pub col: u32, // column
    pub cell_type: CellType,

    pub string_val: String,
    pub integer_val: i64,
    pub float_val: f64,

    //
    pub is_key_column: bool,
}

impl CellData {
    ///
    pub fn new() -> Self {
        Self {
            row: 0,
            col: 0,
            cell_type: CellType::Null,

            string_val: "".to_owned(),
            integer_val: 0_i64,
            float_val: 0_f64,

            is_key_column: false,
        }
    }

    ///
    pub fn to_json(&self) -> Json {
        match self.cell_type {
            CellType::String => {
                //
                Json::from(self.string_val.as_str())
            }
            CellType::Integer => {
                //
                Json::from(self.integer_val)
            }
            CellType::Float => {
                //
                json_util::float_to_json(self.float_val)
            }
            _ => {
                // null
                Json::Null
            }
        }
    }
}
