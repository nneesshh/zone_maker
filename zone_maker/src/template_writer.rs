use std::path::PathBuf;

use handlebars::handlebars_helper;



///
pub fn write_one_zone(
    zone: &str,
    output_path: &PathBuf,
    template_contents: &str,
    data: &serde_json::Map<String, serde_json::Value>,
) {
    //
    let mut reg = handlebars::Handlebars::new();
    reg.set_strict_mode(true);

    // helper -- eval add
    handlebars_helper!(add: |x: Json, y: Json| {
        //
        json_value_eval_add(x, y)
    });
    reg.register_helper("add", Box::new(add));
    
    reg.register_template_string("zone_xml", template_contents)
        .unwrap();

    let zone_xml = reg.render("zone_xml", data).unwrap();

    // write to file
    let output_prefix = output_path.parent().unwrap();
    let output_base = output_path.file_name().unwrap();
    let outpub_ext = output_path.extension().unwrap();
    std::fs::create_dir_all(output_prefix).unwrap();

    let mut full_path = PathBuf::from(output_prefix);
    let file_name = std::format!(
        "{}_{}.{}",
        output_base.to_str().unwrap(),
        zone,
        outpub_ext.to_str().unwrap()
    );
    full_path.push(file_name);
    std::fs::write(full_path, zone_xml.as_bytes()).unwrap();
}

///
#[allow(dead_code)]
pub fn json_value_to_string(val: &serde_json::Value) -> String {
    match val {
        serde_json::Value::Null => {
            //
            "".to_owned()
        }
        serde_json::Value::Bool(b) => {
            //
            if *b {
                "true".to_owned()
            } else {
                "false".to_owned()
            }
        }
        serde_json::Value::Number(n) => {
            //
            if n.is_u64() || n.is_i64() {
                n.to_string()
            } else {
                // without tailing ".0"
                let f = n.as_f64().unwrap();
                f.to_string()
            }
        }
        serde_json::Value::String(s) => {
            //
            s.clone()
        }
        serde_json::Value::Array(v) => {
            //
            std::format!("{:?}", v)
        }
        serde_json::Value::Object(o) => {
            //
            std::format!("{:?}", o)
        }
    }
}

///
#[allow(dead_code)]
pub fn json_value_to_int64(val: &serde_json::Value) -> i64 {
    match val {
        serde_json::Value::Null => {
            //
            0_i64
        }
        serde_json::Value::Bool(b) => {
            //
            if *b {
                1_i64
            } else {
                0_i64
            }
        }
        serde_json::Value::Number(n) => {
            //
            if n.is_u64() || n.is_i64() {
                n.as_i64().unwrap()
            } else {
                // without tailing ".0"
                let f = n.as_f64().unwrap();
                f as i64
            }
        }
        serde_json::Value::String(s) => {
            //
            let n_opt = s.parse::<i64>();
            if let Ok(n) = n_opt {
                n
            } else {
                0_i64
            }
        }
        serde_json::Value::Array(_v) => {
            //
            0_i64
        }
        serde_json::Value::Object(_o) => {
            //
            0_i64
        }
    }
}

///
#[allow(dead_code)]
pub fn json_value_to_double(val: &serde_json::Value) -> f64 {
    match val {
        serde_json::Value::Null => {
            //
            0_f64
        }
        serde_json::Value::Bool(b) => {
            //
            if *b {
                1_f64
            } else {
                0_f64
            }
        }
        serde_json::Value::Number(n) => {
            //
            if n.is_u64() || n.is_i64() {
                let m = n.as_i64().unwrap();
                m as f64
            } else {
                n.as_f64().unwrap()
            }
        }
        serde_json::Value::String(s) => {
            //
            let f_opt = s.parse::<f64>();
            if let Ok(f) = f_opt {
                f
            } else {
                0_f64
            }
        }
        serde_json::Value::Array(_v) => {
            //
            0_f64
        }
        serde_json::Value::Object(_o) => {
            //
            0_f64
        }
    }
}

///
pub fn json_value_eval_add(x: &serde_json::Value, y: &serde_json::Value)-> String {
    if x.is_i64() || x.is_u64() {
        let a = json_value_to_int64(x);
        let b = json_value_to_int64(y);
        std::format!("{}", a + b)
    } else if x.is_f64() || x.is_string() {
        let a = json_value_to_double(x);
        let b = json_value_to_double(y);
        std::format!("{}", a + b)
    } else {
        std::panic!("Cannot add x:{:?} with y:{:?}!!!", x, y)
    }
}