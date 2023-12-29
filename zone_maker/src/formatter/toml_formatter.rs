use std::{
    io::{BufReader, Read},
    path::PathBuf,
};

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
pub struct TomlFormatter {
    config: Config,
    template_contents: String,
    output_path: PathBuf,
}

impl TomlFormatter {
    ///
    pub fn new(ini_path: PathBuf, tpl_path: PathBuf, output_path: PathBuf) -> Self {
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
        let config: Config = toml::from_str(ini_contents.as_str()).unwrap();

        Self {
            config,
            template_contents,
            output_path,
        }
    }

    ///
    pub fn format(&mut self) {
        //
        let mut reg = handlebars::Handlebars::new();
        reg.set_strict_mode(true);
        reg.register_template_string("zone_xml", self.template_contents.as_str())
            .unwrap();

        //
        let mut data = serde_json::value::Map::new();
        data.insert("config".to_owned(), handlebars::to_json(&self.config));
        let zone_xml = reg.render("zone_xml", &data).unwrap();

        // write to file
        let output_prefix = self.output_path.parent().unwrap();
        std::fs::create_dir_all(output_prefix).unwrap();

        std::fs::write(&self.output_path, zone_xml.as_bytes()).unwrap();
    }
}
