use serde_json::Value as Json;

use crate::data_source::DataSource;
use crate::db_access::{MySqlAccess, MySqlAddr};
use crate::field_aliase::AliaseMapper;
use crate::sqls;
use crate::utils::json_row_patcher::JsonRowPatcher;
use crate::utils::json_rows::{JsonRow, JsonRows};
use crate::utils::mysql_util;

///
pub struct MySqlDataSource {
    json_rows: JsonRows,
}

impl MySqlDataSource {
    ///
    pub fn new(key_name: &str, j_patch: &str, db_addr: MySqlAddr) -> Self {
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

        let json_rows = read_rows_from_db(key_name, j_patch, &mut db);

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
        for (_row_idx, json_row) in &self.json_rows.row_table {
            //
            v.push(&json_row.value_table);
        }
        v
    }
}

fn read_rows_from_db(key_name: &str, j_patch: &str, db: &mut MySqlAccess) -> JsonRows {
    let ret = db.exec_prepared_query(sqls::SQL_QUERY_ZONE_ALL, || None);
    assert!(ret.is_ok());

    //
    let aliase_mapper = AliaseMapper::new();
    let json_row_patcher = JsonRowPatcher::new(j_patch);

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

            json_row
                .value_table
                .insert(key.to_owned(), mysql_util::to_json(val));
        }

        // update aliase
        aliase_mapper.update(&mut json_row.value_table);

        // row is not empty
        let val_of_key_opt = json_row.get_value_as_string(key_name);
        if let Some(val_of_key) = val_of_key_opt {
            // collect row
            json_rows.row_table.insert(row_idx, json_row);

            // collect val_of_key
            json_rows.key_2_row_table.insert(val_of_key, row_idx);
        } else {
            let err_msg = std::format!(
                "[row={}] json row get_value() failed!!! key: {} not found!!!",
                row_idx,
                key_name
            );
            log::error!("{}", err_msg);
            std::panic!("{}", err_msg);
        }

        //
        row_idx += 1;
    }

    //
    json_row_patcher.update(key_name, &mut json_rows);

    json_rows
}
