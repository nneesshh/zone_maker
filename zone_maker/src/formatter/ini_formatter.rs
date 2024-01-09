use std::path::PathBuf;

use crate::data_source::{ini_data_source::IniDataSource, DataSource};
use crate::template_helper::{read_template_contents, write_one_zone};
use crate::utils::json_util;

///
pub struct IniFormatter {
    zone_id: i32,

    data_source: IniDataSource,

    template_contents: String,
    output_path: PathBuf,
}

impl IniFormatter {
    ///
    pub fn new(key_name: &str, ini_path: PathBuf, tpl_path: PathBuf, output_path: PathBuf) -> Self {
        // ds -- ini, read toml to field table
        let data_source = IniDataSource::new(&ini_path);

        // tpl -- read template to contents
        let template_contents = read_template_contents(&tpl_path);

        // zone id
        let mut zone_id = 0_i32;
        let zone_opt = data_source.get(key_name);
        if let Some(zone) = zone_opt {
            zone_id = json_util::to_int64(zone) as i32;
        }

        Self {
            zone_id,

            data_source,

            template_contents,
            output_path,
        }
    }

    /// return full_path
    pub fn format(&self) {
        //
        let zone = self.zone_id.to_string();

        // only one row
        let data_vec = self.data_source.get_all_rows();
        assert_eq!(data_vec.len(), 1);

        let data = data_vec[0];

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
