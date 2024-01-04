use std::path::PathBuf;

use handlebars::handlebars_helper;

use crate::json_helper;

///
pub fn write_one_zone(
    zone: &str,
    output_path: &PathBuf,
    template_contents: &str,
    data: &serde_json::Map<String, serde_json::Value>,
    strict_mode: bool,
) -> std::path::PathBuf {
    //
    let mut reg = handlebars::Handlebars::new();
    reg.set_strict_mode(strict_mode);

    // helper -- eval add
    handlebars_helper!(add: |x: Json, y: Json| {
        //
        json_helper::eval_add(x, y)
    });
    reg.register_helper("add", Box::new(add));

    reg.register_template_string("zone_xml", template_contents)
        .unwrap();

    let zone_xml = reg.render("zone_xml", data).unwrap();

    // write to file
    let parent_path = output_path.parent().unwrap();
    let file_name_path = output_path.file_name().unwrap();

    // drop file name ext
    let file_name = file_name_path.to_str().unwrap();
    let mut file_name_parts = file_name.split(".").collect::<Vec<&str>>();
    let ext_opt = file_name_parts.pop();
    let ext = ext_opt.unwrap_or("xml");

    // base
    let mut base = "".to_owned();
    for part in file_name_parts {
        if !part.is_empty() {
            base.push_str(part);
            base.push_str("_");
        }
    }

    // ensure parent dir
    std::fs::create_dir_all(parent_path).unwrap();

    let mut full_path = PathBuf::from(parent_path);
    let file_name = std::format!("{}{}.{}", base, zone, ext);
    full_path.push(file_name);
    std::fs::write(&full_path, zone_xml.as_bytes()).unwrap();

    full_path
}
