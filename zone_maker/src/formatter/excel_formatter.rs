use std::{
    io::{BufReader, Read},
    path::PathBuf,
};

use calamine::{open_workbook, DataType, Reader, Xlsx};

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
pub struct ExcelFormatter {
    config: Config,
    template_contents: String,
    output_path: PathBuf,
}

impl ExcelFormatter {
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

fn read_config_from_xlsx(path: &PathBuf) {
    // opens a new workbook
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");

    // Read whole worksheet data and provide some statistics
    if let Ok(range) = workbook.worksheet_range("Sheet1") {
        let total_cells = range.get_size().0 * range.get_size().1;
        let non_empty_cells: usize = range.used_cells().count();
        println!(
            "Found {} cells in 'Sheet1', including {} non empty cells",
            total_cells, non_empty_cells
        );
        // alternatively, we can manually filter rows
        assert_eq!(
            non_empty_cells,
            range
                .rows()
                .flat_map(|r| r.iter().filter(|&c| c != &DataType::Empty))
                .count()
        );
    }

    // Check if the workbook has a vba project
    if let Some(Ok(mut vba)) = workbook.vba_project() {
        let vba = vba.to_mut();
        let module1 = vba.get_module("Module 1").unwrap();
        println!("Module 1 code:");
        println!("{}", module1);
        for r in vba.get_references() {
            if r.is_missing() {
                println!("Reference {} is broken or not accessible", r.name);
            }
        }
    }

    // You can also get defined names definition (string representation only)
    for name in workbook.defined_names() {
        println!("name: {}, formula: {}", name.0, name.1);
    }

    // Now get all formula!
    let sheets = workbook.sheet_names().to_owned();
    for s in sheets {
        println!(
            "found {} formula in '{}'",
            workbook
                .worksheet_formula(&s)
                .expect("error while getting formula")
                .rows()
                .flat_map(|r| r.iter().filter(|f| !f.is_empty()))
                .count(),
            s
        );
    }
}
