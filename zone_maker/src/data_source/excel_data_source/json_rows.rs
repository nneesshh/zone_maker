use serde_json::Value as Json;

use crate::utils::json_util;

///
pub struct JsonRows {
    pub key_2_row_table: hashbrown::HashMap<String, u32>, // key 2 row idx
    pub row_table: hashbrown::HashMap<u32, JsonRow>,      // row idx 2 row values
}

impl JsonRows {
    ///
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.key_2_row_table.len()
    }

    ///
    #[allow(dead_code)]
    pub fn get_row(&self, row: u32) -> Option<&JsonRow> {
        self.row_table.get(&row)
    }

    ///
    #[allow(dead_code)]
    pub fn get_row_by_key(&self, key: &str) -> Option<&JsonRow> {
        let row_opt = self.key_2_row_table.get(key);
        if let Some(row) = row_opt {
            self.row_table.get(row)
        } else {
            None
        }
    }
}

///
pub struct JsonRow {
    pub value_table: serde_json::Map<String, Json>,
}

impl JsonRow {
    ///
    pub fn new() -> Self {
        Self {
            value_table: serde_json::Map::new(),
        }
    }

    ///
    #[allow(dead_code)]
    pub fn get_value_as_string(&self, key: &str) -> Option<String> {
        if let Some(val) = self.value_table.get(key) {
            let s = json_util::to_string(val);
            Some(s)
        } else {
            //
            None
        }
    }

    ///
    #[allow(dead_code)]
    pub fn get_value_as_int64(&self, key: &str) -> Option<i64> {
        if let Some(val) = self.value_table.get(key) {
            let n = json_util::to_int64(val);
            Some(n)
        } else {
            //
            None
        }
    }

    ///
    #[allow(dead_code)]
    pub fn get_value_as_double(&self, key: &str) -> Option<f64> {
        if let Some(val) = self.value_table.get(key) {
            let f = json_util::to_double(val);
            Some(f)
        } else {
            //
            None
        }
    }
}
