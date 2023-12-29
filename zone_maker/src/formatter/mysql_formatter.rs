use std::io::{BufReader, Read};
use std::path::PathBuf;

use handlebars::handlebars_helper;

use crate::db_access::{MySqlAccess, MySqlAddr, SqlPreparedParams};
use crate::sqls;

///
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
    zone_id: u64,
    group_id: u32,
    gw_port: u16,
    http_port: u16,
    db_url: String,
    redis_ip: String,
    redis_port: u16,
    web_url: String,
}

///
pub struct MySqlFormatter {
    db_addr: MySqlAddr,
    template_contents: String,
    output_path: PathBuf,
}

impl MySqlFormatter {
    ///
    pub fn new(db_addr: MySqlAddr, tpl_path: PathBuf, output_path: PathBuf) -> Self {
        // tpl -- read template to contents
        let tpl_file = std::fs::File::open(&tpl_path).unwrap();
        let mut template_buf_reader = BufReader::new(tpl_file);
        let mut template_contents = String::new();
        template_buf_reader
            .read_to_string(&mut template_contents)
            .unwrap();

        Self {
            db_addr,
            template_contents,
            output_path,
        }
    }

    ///
    pub fn format(&mut self) {
        //
        let mut reg = handlebars::Handlebars::new();
        reg.set_strict_mode(true);
        handlebars_helper!(concat: |x: u64, y: u64| {
            //
            std::format!("{}{}", x, y)
        });

        //
        reg.register_helper("concat", Box::new(concat));

        reg.register_template_string("zone_xml", self.template_contents.as_str())
            .unwrap();

        // read config from db
        let config: Config = read_config_from_db(&self.db_addr).unwrap();

        //
        let mut data = serde_json::value::Map::new();
        data.insert("config".to_owned(), handlebars::to_json(&config));
        let zone_xml = reg.render("zone_xml", &data).unwrap();

        // write to file
        let output_prefix = self.output_path.parent().unwrap();
        std::fs::create_dir_all(output_prefix).unwrap();

        std::fs::write(&self.output_path, zone_xml.as_bytes()).unwrap();
    }
}

fn read_config_from_db(db_addr: &MySqlAddr) -> Option<Config> {
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

    let ret = db.exec_prepared_query(sqls::SQL_QUERY_ZONE, || {
        //
        let mut params = SqlPreparedParams::new();
        params.add_string("5001");

        params
    });
    assert!(ret.is_ok());

    let rows = ret.unwrap();
    if rows.len() > 0 {
        let row = &rows[0];
        let mut idx: usize = 0;
        let zone_id = row.get_uint64(&mut idx).unwrap();
        let group_id = row.get_uint64(&mut idx).unwrap_or(0);
        let gw_port = row.get_uint64(&mut idx).unwrap();
        let http_port = row.get_uint64(&mut idx).unwrap();

        let db_url = row.get_string(&mut idx).unwrap();

        let redis_ip = row.get_string(&mut idx).unwrap();
        let redis_port = row.get_uint64(&mut idx).unwrap();

        let web_url = row.get_string(&mut idx).unwrap();

        let _test_blob = row.get_blob_by_name("test_blob");
        let _test_timestamp = row.get_timestamp_by_name("test_timestamp");

        let config = Config {
            zone_id,
            group_id: group_id as u32,
            gw_port: gw_port as u16,
            http_port: http_port as u16,
            db_url: db_url.to_owned(),
            redis_ip: redis_ip.to_owned(),
            redis_port: redis_port as u16,
            web_url: web_url.to_owned(),
        };

        Some(config)
    } else {
        None
    }
}
