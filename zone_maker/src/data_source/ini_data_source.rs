use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::str::FromStr;

use serde_json::Value as Json;
use toml::Value as Toml;

use crate::data_source::DataSource;
use crate::field_aliase::AliaseMapper;
use crate::utils::toml_util;

///
pub struct IniDataSource {
    data: serde_json::Map<String, Json>,
}

impl IniDataSource {
    ///
    pub fn new(ini_path: &PathBuf) -> Self {
        //
        let ini_file = std::fs::File::open(&ini_path).unwrap();
        let mut ini_buf_reader = BufReader::new(ini_file);
        let mut ini_contents = String::new();
        ini_buf_reader.read_to_string(&mut ini_contents).unwrap();

        //
        let toml_field_table = toml::Table::from_str(ini_contents.as_str()).unwrap();

        Self {
            data: toml_to_json(&toml_field_table),
        }
    }

    ///
    pub fn get(&self, key: &str) -> Option<&Json> {
        let val_opt = self.data.get(key);
        if let Some(val) = val_opt {
            Some(val)
        } else {
            None
        }
    }
}

impl DataSource for IniDataSource {
    ///
    fn get_row(&self, key: &str) -> Option<&serde_json::Map<String, Json>> {
        if self.get(key).is_some() {
            Some(&self.data)
        } else {
            None
        }
    }

    ///
    fn get_all_rows(&self) -> Vec<&serde_json::Map<String, Json>> {
        let mut v = Vec::new();
        v.push(&self.data);
        v
    }
}

fn toml_to_json(field_table: &toml::map::Map<String, Toml>) -> serde_json::Map<String, Json> {
    // prepare data
    let mut data = serde_json::Map::new();
    for (key, val) in field_table {
        //
        data.insert(key.clone(), toml_util::to_json(val));
    }

    // update aliase
    let aliase_mapper = AliaseMapper::new();
    aliase_mapper.update(&mut data);

    data
}
