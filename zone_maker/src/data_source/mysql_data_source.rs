use serde_json::Value as Json;

use crate::data_source::excel_data_source::json_rows::JsonRow;
use crate::data_source::DataSource;
use crate::db_access::{MySqlAccess, MySqlAddr};
use crate::field_aliase::AliaseMapper;
use crate::sqls;
use crate::utils::mysql_util;

use super::excel_data_source::json_rows::JsonRows;

///
pub struct MySqlDataSource {
    json_rows: JsonRows,
}

impl MySqlDataSource {
    ///
    pub fn new(key_name: &str, db_addr: MySqlAddr) -> Self {
        let url = std::format!(
            "mysql://{}:{}@{}:{}/{}",
            db_addr.user,
            db_addr.password,
            db_addr.host,
            db_addr.port,
            db_addr.dbname,
        );
        let mut db = MySqlAccess::new(url.as_str());
        match db.open() {
            Ok(_) => {
                //
            }
            Err(err) => {
                //
                std::panic!("{}", err);
            }
        };

        let json_rows = read_rows_from_db(key_name, &mut db);

        Self { json_rows }
    }
}

impl DataSource for MySqlDataSource {
    ///
    fn get_row(&self, key: &str) -> Option<&serde_json::Map<String, Json>> {
        let json_row_opt = self.json_rows.get_row_by_key(key);
        if let Some(json_row) = json_row_opt {
            //
            Some(&json_row.value_table)
        } else {
            //
            None
        }
    }

    ///
    fn get_all_rows(&self) -> Vec<&serde_json::Map<String, Json>> {
        let mut v = Vec::with_capacity(self.json_rows.len());
        for (_row, json_row) in &self.json_rows.row_table {
            //
            v.push(&json_row.value_table);
        }
        v
    }
}

fn read_rows_from_db(key_name: &str, db: &mut MySqlAccess) -> JsonRows {
    let ret = db.exec_prepared_query(sqls::SQL_QUERY_ZONE_ALL, || None);
    assert!(ret.is_ok());

    let aliase_mapper = AliaseMapper::new();

    //
    let mut json_rows = JsonRows {
        key_2_row_table: hashbrown::HashMap::new(),
        row_table: hashbrown::HashMap::new(),
    };

    let sql_rows = ret.unwrap();
    let mut row_idx = 0_u32;
    for row in &sql_rows.rows {
        let mut json_row = JsonRow {
            value_table: serde_json::Map::new(),
        };
        for column in &sql_rows.columns {
            let key = column.as_str();
            let val = row.get_by_name(key);

            // it is key
            if key == key_name {
                // collect val_of_key
                let val_of_key = mysql_util::to_string(val);
                json_rows.key_2_row_table.insert(val_of_key, row_idx);
            }

            // update aliase
            aliase_mapper.update(&mut json_row.value_table);

            json_row
                .value_table
                .insert(key.to_owned(), mysql_util::to_json(val));
        }

        // collect row
        json_rows.row_table.insert(row_idx, json_row);

        //
        row_idx += 1;
    }

    json_rows
}
