use std::io::{BufReader, Read};
use std::str::FromStr;

use serde_json::Value as Json;

use crate::utils::toml_util;

pub struct AliaseMapper {
    pub aliase_table: hashbrown::HashMap<String, String>,
}

impl AliaseMapper {
    ///
    pub fn new() -> Self {
        //
        let mut aliase_table = hashbrown::HashMap::new();

        // ini -- read toml to contents
        let path = std::path::PathBuf::from("res/field_aliase.ini");
        let ini_file = std::fs::File::open(path).unwrap();
        let mut ini_buf_reader = BufReader::new(ini_file);
        let mut ini_contents = String::new();
        ini_buf_reader.read_to_string(&mut ini_contents).unwrap();

        //
        let toml_table = toml::Table::from_str(ini_contents.as_str()).unwrap();
        for (key, toml_val) in toml_table {
            aliase_table.insert(key, toml_util::to_string(&toml_val));
        }

        Self { aliase_table }
    }

    ///
    #[allow(dead_code)]
    pub fn get_aliase_name(&self, key: &str) -> Option<String> {
        self.aliase_table.get(key).map(|v| {
            //
            v.to_string()
        })
    }

    ///
    pub fn update(&self, data: &mut serde_json::Map<String, Json>) {
        //
        for (key, val) in &self.aliase_table {
            let field_aliase = val.as_str();
            let source_val_opt = data.get(key.as_str());
            if let Some(source_val) = source_val_opt {
                // insert (aliase, value) pair
                data.insert(field_aliase.to_owned(), source_val.clone());
            } else {
                let aliase_val_opt = data.get(field_aliase);
                if let Some(aliase_val) = aliase_val_opt {
                    // insert (aliase, value) pair
                    data.insert(key.to_owned(), aliase_val.clone());
                }
            }
        }
    }
}
