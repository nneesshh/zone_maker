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
    pub fn to_json_value(&self) -> serde_json::Value {
        match self.cell_type {
            CellType::String => {
                //
                serde_json::Value::from(self.string_val.as_str())
            }
            CellType::Integer => {
                //
                serde_json::Value::from(self.integer_val)
            }
            CellType::Float => {
                //
                let n_floor = self.float_val.floor();
                if n_floor == self.float_val {
                    // to i64
                    serde_json::Value::from(n_floor as i64)
                } else {
                    serde_json::Value::from(self.float_val)
                }
            }
            _ => {
                // null
                serde_json::Value::Null
            }
        }
    }
}

/*
// formats like "2001-07-08T00:34:60.026490+09:30"
fn timestamp_millis_from_iso8601(s: &str) -> i64 {
    let dt = DateTime::parse_from_rfc3339(s).unwrap();
    dt.timestamp_millis()
}

fn as_int64(cell: DataType) -> Option<i64> {
    match cell {
        DataType::Int(n) => {
            //
            Some(n)
        }
        DataType::Float(f) => {
            //
            Some(f as i64)
        }
        DataType::String(s) => {
            //
            let n = s.as_str().parse()<i64>().unwrap();
            Some(n)
        }
        DataType::Bool(b) => {
            //
            Some(b as i64)
        }
        DataType::DateTime(f) => {
            //
            Some(f as i64)
        }
        DataType::Duration(f) => {
            //
            Some(f as i64)
        }
        // Date, Time or DateTime in ISO 8601
        DataType::DateTimeIso(s) => {
            //
            let millis = timestamp_millis_from_iso8601(s.as_str());
            Some(millis)
        }
        // Duration in ISO 8601
        DataType::DurationIso(s) => {
            //
            let ret = s.as_str().parse::<iso8601_duration::Duration>();
            if let Ok(d) = ret {
                //
                Some(d.num_seconds().unwrap() as i64)
            }
            else {
                //
                Some(0_i64)
            }
        }
        // Error
        DataType::Error(CellErrorType) => {
            //
            None
        }
        // Empty cell
        DataType::Empty => {
            //
            None
        }
    }
}

fn as_string(cell: DataType) -> Option<String> {
    match cell {
        DataType::Int(n) => {
            //
            Some(n.to_string())
        }
        DataType::Float(f) => {
            //
            Some(f.to_string())
        }
        DataType::String(s) => {
            //
            Some(s)
        }
        DataType::Bool(b) => {
            //
            if b {
                Some("1".to_owned())
            }
            else {
                Some("0".to_owned())
            }
        }
        DataType::DateTime(f) => {
            //
            Some(f.to_string())
        }
        DataType::Duration(f) => {
            //
            Some(f.to_string())
        }
        // Date, Time or DateTime in ISO 8601
        DataType::DateTimeIso(s) => {
            //
            Some(s)
        }
        // Duration in ISO 8601
        DataType::DurationIso(s) => {
            //
                Some(s)
        }
        // Error
        DataType::Error(CellErrorType) => {
            //
            None
        }
        // Empty cell
        DataType::Empty => {
            //
            None
        }
    }
}
 */
