use std::{
    io::{BufReader, Read},
    path::PathBuf,
    str::FromStr,
};

use crate::template_writer::write_one_zone;
use crate::{field_aliase::AliaseMapper, toml_helper};

///
pub struct TomlFormatter {
    key_name: String,
    field_table: toml::Table,
    template_contents: String,
    output_path: PathBuf,
}

impl TomlFormatter {
    ///
    pub fn new(key_name: &str, ini_path: PathBuf, tpl_path: PathBuf, output_path: PathBuf) -> Self {
        // ini -- read toml to contents
        let ini_file = std::fs::File::open(&ini_path).unwrap();
        let mut ini_buf_reader = BufReader::new(ini_file);
        let mut ini_contents = String::new();
        ini_buf_reader.read_to_string(&mut ini_contents).unwrap();

        // tpl -- read template to contents
        let tpl_file = std::fs::File::open(&tpl_path).unwrap();
        let mut template_buf_reader = BufReader::new(tpl_file);
        let mut template_contents = String::new();
        template_buf_reader
            .read_to_string(&mut template_contents)
            .unwrap();

        //
        let field_table = toml::Table::from_str(ini_contents.as_str()).unwrap();

        Self {
            key_name: key_name.to_owned(),
            field_table,
            template_contents,
            output_path,
        }
    }

    /// return full_path
    pub fn format(&self) {
        // prepare data
        let mut data = serde_json::Map::new();
        for (key, val) in &self.field_table {
            //
            data.insert(key.clone(), toml_helper::to_json(val));
        }

        // update aliase
        let aliase_mapper = AliaseMapper::new();
        aliase_mapper.update(&mut data);

        //
        let mut zone_id = 0_i32;
        let zone_opt = self.field_table.get(self.key_name.as_str());
        if let Some(zone) = zone_opt {
            zone_id = zone.as_integer().unwrap() as i32;
        }
        let zone = zone_id.to_string();

        //
        let _ = write_one_zone(
            zone.as_str(),
            &self.output_path,
            self.template_contents.as_str(),
            &data,
            false,
        );
    }
}
