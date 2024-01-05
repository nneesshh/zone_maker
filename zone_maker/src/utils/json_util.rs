use serde_json::Value as Json;

///
#[allow(dead_code)]
pub fn to_string(val: &Json) -> String {
    match val {
        Json::Null => {
            //
            "".to_owned()
        }
        Json::Bool(b) => {
            //
            if *b {
                "true".to_owned()
            } else {
                "false".to_owned()
            }
        }
        Json::Number(n) => {
            //
            if n.is_u64() || n.is_i64() {
                n.to_string()
            } else {
                // without tailing ".0"
                let f = n.as_f64().unwrap();
                f.to_string()
            }
        }
        Json::String(s) => {
            //
            s.clone()
        }
        Json::Array(v) => {
            //
            std::format!("{:?}", v)
        }
        Json::Object(o) => {
            //
            std::format!("{:?}", o)
        }
    }
}

///
#[allow(dead_code)]
pub fn to_int64(val: &Json) -> i64 {
    match val {
        Json::Null => {
            //
            0_i64
        }
        Json::Bool(b) => {
            //
            if *b {
                1_i64
            } else {
                0_i64
            }
        }
        Json::Number(n) => {
            //
            if n.is_u64() || n.is_i64() {
                n.as_i64().unwrap()
            } else {
                // without tailing ".0"
                let f = n.as_f64().unwrap();
                f as i64
            }
        }
        Json::String(s) => {
            //
            let n_opt = s.parse::<i64>();
            if let Ok(n) = n_opt {
                n
            } else {
                0_i64
            }
        }
        Json::Array(_v) => {
            //
            0_i64
        }
        Json::Object(_o) => {
            //
            0_i64
        }
    }
}

///
#[allow(dead_code)]
pub fn to_double(val: &Json) -> f64 {
    match val {
        Json::Null => {
            //
            0_f64
        }
        Json::Bool(b) => {
            //
            if *b {
                1_f64
            } else {
                0_f64
            }
        }
        Json::Number(n) => {
            //
            if n.is_u64() || n.is_i64() {
                let m = n.as_i64().unwrap();
                m as f64
            } else {
                n.as_f64().unwrap()
            }
        }
        Json::String(s) => {
            //
            let f_opt = s.parse::<f64>();
            if let Ok(f) = f_opt {
                f
            } else {
                0_f64
            }
        }
        Json::Array(_v) => {
            //
            0_f64
        }
        Json::Object(_o) => {
            //
            0_f64
        }
    }
}

/// Erase .0
pub fn float_to_json(f: f64) -> Json {
    let n_floor = f.floor();
    if n_floor == f {
        // to i64
        Json::from(n_floor as i64)
    } else {
        Json::from(f)
    }
}

///
pub fn eval_add(x: &Json, y: &Json) -> String {
    if x.is_i64() || x.is_u64() {
        let a = to_int64(x);
        let b = to_int64(y);
        std::format!("{}", a + b)
    } else if x.is_f64() || x.is_string() {
        let a = to_double(x);
        let b = to_double(y);
        std::format!("{}", a + b)
    } else {
        std::panic!("Cannot add x:{:?} with y:{:?}!!!", x, y)
    }
}
