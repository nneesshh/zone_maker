
///
pub struct JsonRows {
    pub key_2_row_table: hashbrown::HashMap<String, u32>, // key 2 row idx
    pub row_table: hashbrown::HashMap<u32, JsonRow>,      // row idx 2 row values
}

impl JsonRows {
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
    pub value_table: serde_json::Map<String, serde_json::Value>,
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
            match val {
                serde_json::Value::Null => {
                    //
                    Some("".to_owned())
                }
                serde_json::Value::Bool(b) => {
                    //
                    if *b {
                        Some("true".to_owned())
                    } else {
                        Some("false".to_owned())
                    }
                }
                serde_json::Value::Number(n) => {
                    //
                    if n.is_u64() || n.is_i64() {
                        Some(n.to_string())
                    } else {
                        // without tailing ".0"
                        let f = n.as_f64().unwrap();
                        Some(f.to_string())
                    }
                }
                serde_json::Value::String(s) => {
                    //
                    Some(s.clone())
                }
                serde_json::Value::Array(v) => {
                    //
                    Some(std::format!("{:?}", v))
                }
                serde_json::Value::Object(o) => {
                    //
                    Some(std::format!("{:?}", o))
                }
            }
        } else {
            //
            None
        }
    }
}
