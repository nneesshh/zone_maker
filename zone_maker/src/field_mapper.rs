use std::{
    io::{BufReader, Read},
    path::PathBuf,
    str::FromStr,
};

use toml::{Table, Value};

pub struct FieldMapper {
    pub field_table: Table,
}

impl FieldMapper {
    ///
    pub fn new(path: PathBuf) -> Self {
        // ini -- read toml to contents
        let ini_file = std::fs::File::open(&path).unwrap();
        let mut ini_buf_reader = BufReader::new(ini_file);
        let mut ini_contents = String::new();
        ini_buf_reader.read_to_string(&mut ini_contents).unwrap();

        //
        let field_table = toml::Table::from_str(ini_contents.as_str()).unwrap();

        Self { field_table }
    }

    ///
    pub fn get_field_name(&self, key: &str) -> Option<String> {
        self.field_table.get(key).map(|v| {
            //
            v.to_string()
        })
    }
}
