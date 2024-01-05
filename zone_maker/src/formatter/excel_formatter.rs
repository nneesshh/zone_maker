use std::path::PathBuf;

use crate::data_source::excel_data_source::ExcelDataSource;
use crate::data_source::DataSource;
use crate::template_helper::{read_template_contents, write_one_zone};
use crate::utils::json_util;

///
pub struct ExcelFormatter {
    key_name: String,
    zone_id: i32,

    data_source: ExcelDataSource,

    template_contents: String,
    output_path: PathBuf,
}

impl ExcelFormatter {
    ///
    pub fn new(
        key_name: &str,
        zone_id: i32,
        xlsx_path: PathBuf,
        tpl_path: PathBuf,
        output_path: PathBuf,
    ) -> Self {
        // ds -- read excel file to json rows
        let data_source = ExcelDataSource::new(key_name, &xlsx_path);

        // tpl -- read template to contents
        let template_contents = read_template_contents(&tpl_path);

        //
        Self {
            key_name: key_name.to_owned(),
            zone_id,

            data_source,

            template_contents,
            output_path,
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
                    true,
                );
            } else {
                log::error!("Zone {} not found", zone);
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
                        true,
                    );
                } else {
                    log::error!("Key {} field is not found among row!!!", key);
                }
            }
        }
    }
}
