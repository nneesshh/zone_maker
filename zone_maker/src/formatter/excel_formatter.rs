use std::io::{BufReader, Read};
use std::path::PathBuf;

use crate::template_writer::{write_one_zone, ZONE_ID};

use self::json_rows::JsonRows;

mod excel_cell_data;
mod excel_field_info;
mod excel_range_helper;
mod excel_rows;
mod json_rows;

///
pub struct ExcelFormatter {
    zone_id: i32,
    json_rows: JsonRows,
    template_contents: String,
    output_path: PathBuf,
}

impl ExcelFormatter {
    ///
    pub fn new(zone: &str, xlsx_path: PathBuf, tpl_path: PathBuf, output_path: PathBuf) -> Self {
        // excel -- read excel file to json rows
        let json_rows = excel_range_helper::read_json_rows_from_xlsx(&xlsx_path);

        // tpl -- read template to contents
        let tpl_file = std::fs::File::open(&tpl_path).unwrap();
        let mut template_buf_reader = BufReader::new(tpl_file);
        let mut template_contents = String::new();
        template_buf_reader
            .read_to_string(&mut template_contents)
            .unwrap();

        //
        let mut zone_id = 0_i32;
        let zone_ret = zone.parse::<i32>();
        if let Ok(zone) = zone_ret {
            zone_id = zone;
        }

        //
        Self {
            zone_id,
            json_rows,
            template_contents,
            output_path,
        }
    }

    ///
    pub fn format(&mut self) {
        //
        if self.zone_id > 0 {
            // output one zone
            let zone = self.zone_id.to_string();
            let json_row_opt = self.json_rows.get_row_by_key(&zone);
            if let Some(json_row) = json_row_opt {
                //
                let data = &json_row.value_table;
                write_one_zone(
                    zone.as_str(),
                    &self.output_path,
                    self.template_contents.as_str(),
                    data,
                );
            } else {
                log::error!("Zone {} not found", zone);
            }
        } else {
            let zone = ZONE_ID.to_owned();

            // output all zones
            for (_row, json_row) in &self.json_rows.row_table {
                //
                let zone_opt = json_row.get_value_as_string(&zone);
                if let Some(zone) = zone_opt {
                    //
                    let data = &json_row.value_table;
                    write_one_zone(
                        zone.as_str(),
                        &self.output_path,
                        self.template_contents.as_str(),
                        data,
                    );
                } else {
                    log::error!("Zone {} not found", ZONE_ID);
                }
            }
        }
    }
}
