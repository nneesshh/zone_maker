use serde_json::Value as Json;
use toml::Value as Toml;

///
pub fn to_string(toml_val: &Toml) -> String {
    match toml_val {
        Toml::String(s) => {
            //
            s.clone()
        }
        Toml::Integer(i) => {
            //
            i.to_string()
        }
        Toml::Float(f) => {
            //
            f.to_string()
        }
        Toml::Boolean(b) => {
            //
            if *b {
                "1".to_owned()
            } else {
                "0".to_owned()
            }
        }
        Toml::Array(arr) => {
            //
            std::format!("{:?}", arr)
        }
        Toml::Table(table) => {
            //
            std::format!("{:?}", table)
        }
        Toml::Datetime(dt) => {
            //
            dt.to_string()
        }
    }
}

/// convert toml value to json value
pub fn to_json(toml_val: &Toml) -> Json {
    match toml_val {
        Toml::String(s) => {
            //
            Json::String(s.clone())
        }
        Toml::Integer(i) => {
            //
            Json::Number((*i).into())
        }
        Toml::Float(f) => {
            //
            let n = serde_json::Number::from_f64(*f).expect("float infinite and nan not allowed");
            Json::Number(n)
        }
        Toml::Boolean(b) => {
            //
            Json::Bool(*b)
        }
        Toml::Array(arr) => {
            //
            Json::Array(arr.into_iter().map(to_json).collect())
        }
        Toml::Table(table) => {
            //
            Json::Object(
                table
                    .into_iter()
                    .map(|(k, v)| (k.clone(), to_json(v)))
                    .collect(),
            )
        }
        Toml::Datetime(dt) => {
            //
            Json::String(dt.to_string())
        }
    }
}
