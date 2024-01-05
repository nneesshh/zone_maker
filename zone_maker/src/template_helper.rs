use std::{
    io::{BufReader, Read},
    path::{is_separator, PathBuf, MAIN_SEPARATOR_STR},
};

use serde_json::Value as Json;

use handlebars::handlebars_helper;

use crate::utils::json_util;

const DEFAULT_EXT: &str = "xml";
const TEMPLATE_KEY: &str = "zone_xml";

///
pub fn read_template_contents(tpl_path: &PathBuf) -> String {
    let tpl_file = std::fs::File::open(tpl_path).unwrap();
    let mut reader = BufReader::new(tpl_file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    contents
}

///
pub fn write_one_zone(
    zone: &str,
    output_path: &PathBuf,
    template_contents: &str,
    data: &serde_json::Map<String, Json>,
    strict_mode: bool,
) -> std::path::PathBuf {
    //
    let mut reg = handlebars::Handlebars::new();
    reg.set_strict_mode(strict_mode);

    // helper -- eval add
    handlebars_helper!(add: |x: Json, y: Json| {
        //
        json_util::eval_add(x, y)
    });
    reg.register_helper("add", Box::new(add));

    reg.register_template_string(TEMPLATE_KEY, template_contents)
        .unwrap();

    let zone_xml = reg.render(TEMPLATE_KEY, data).unwrap();

    //
    let (full_path, parent_path) = make_full_path(output_path, zone);

    // ensure parent dir
    std::fs::create_dir_all(&parent_path).unwrap();

    // write to file
    std::fs::write(&full_path, zone_xml.as_bytes()).unwrap();

    log::info!("write zone config to ({:?})", full_path);
    full_path
}

// return (full_path, parent_path)
fn make_full_path(output_path: &PathBuf, zone: &str) -> (std::path::PathBuf, std::path::PathBuf) {
    //
    let mut parent_path_string = "".to_owned();

    let mut base;
    let ext;

    //
    let output_path_str = output_path.to_str().unwrap();
    let the_last_char = output_path_str.chars().last().unwrap();
    if is_separator(the_last_char) {
        // empty file name, with default extension
        parent_path_string = output_path_str.to_owned();
        ext = DEFAULT_EXT.to_owned();
        base = "".to_owned();
    } else {
        // split output path into parts
        let mut parts = output_path_str
            .split(|c| is_separator(c))
            .collect::<Vec<&str>>();

        // parse file name
        let file_name_opt = parts.pop();

        // parse parent
        for part in parts {
            if !part.is_empty() {
                parent_path_string.push_str(part);
                parent_path_string.push_str(MAIN_SEPARATOR_STR);
            }
        }

        // split file name into parts
        let mut file_name_string = "".to_owned();
        if let Some(file_name) = file_name_opt {
            file_name_string = file_name.to_owned();
        }
        let mut file_name_parts = file_name_string.split(".").collect::<Vec<&str>>();

        // parse ext
        let ext_opt = file_name_parts.pop();
        ext = ext_opt
            .map(|s| {
                //
                if s.is_empty() {
                    DEFAULT_EXT.to_owned()
                } else {
                    //
                    s.to_owned()
                }
            })
            .unwrap_or(DEFAULT_EXT.to_owned());

        // base
        base = "".to_owned();
        for part in file_name_parts {
            if !part.is_empty() {
                base.push_str(part);
                base.push_str("_");
            }
        }
    }

    // write to file
    let parent_path = std::path::PathBuf::from(parent_path_string.as_str());
    let mut full_path = PathBuf::from(&parent_path);
    let file_name = std::format!("{}{}.{}", base, zone, ext);
    full_path.push(file_name);

    println!("ext: {}", ext);

    //
    (full_path, parent_path)
}
