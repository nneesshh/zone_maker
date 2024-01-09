use std::path::PathBuf;

use crate::data_source::mysql_data_source::MySqlDataSource;
use crate::data_source::DataSource;
use crate::db_access::MySqlAddr;
use crate::template_helper::{read_template_contents, write_one_zone};
use crate::utils::json_util;

///
pub struct MySqlFormatter {
    key_name: String,
    zone_id: i32,

    data_source: MySqlDataSource,

    template_contents: String,
    output_path: PathBuf,

    strict_mode: bool,
}

impl MySqlFormatter {
    ///
    pub fn new(
        key_name: &str,
        zone_id: i32,
        db_addr: MySqlAddr,
        tpl_path: PathBuf,
        output_path: PathBuf,
    ) -> Self {
        // ds -- excel, read mysql data to json rows
        let data_source = MySqlDataSource::new(key_name, db_addr);

        // tpl -- read template to contents
        let template_contents = read_template_contents(&tpl_path);

        Self {
            key_name: key_name.to_owned(),
            zone_id,

            data_source,

            template_contents,
            output_path,

            strict_mode: false,
        }
    }

    ///
    pub fn format(&self) {
        //
        if self.zone_id > 0 {
            // output one zone
            let zone = self.zone_id.to_string();
            let data_opt = self.data_source.get_row(&zone);
            if let Some(data) = data_opt {
                //
                let _ = write_one_zone(
                    zone.as_str(),
                    &self.output_path,
                    self.template_contents.as_str(),
                    data,
                    self.strict_mode,
                );
            } else {
                let err_msg = std::format!("Zone {} not found", zone);
                log::error!("{}", err_msg);
                std::panic!("{}", err_msg);
            }
        } else {
            // output all zones
            let key = self.key_name.as_str();
            let data_vec = self.data_source.get_all_rows();

            for data in data_vec {
                //
                let zone_opt = data.get(key);
                if let Some(zone) = zone_opt {
                    //
                    let zone = json_util::to_string(zone);

                    //
                    let _ = write_one_zone(
                        zone.as_str(),
                        &self.output_path,
                        self.template_contents.as_str(),
                        data,
                        self.strict_mode,
                    );
                } else {
                    let err_msg = std::format!("Key {} field is not found among row!!!", key);
                    log::error!("{}", err_msg);
                    std::panic!("{}", err_msg);
                }
            }
        }
    }
}
