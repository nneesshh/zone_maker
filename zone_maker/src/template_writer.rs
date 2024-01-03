use std::path::PathBuf;

///
//pub const ZONE_ID:& str = "zone_id";
pub const ZONE_ID: &str = "serverid";

///
pub fn write_one_zone(
    zone: &str,
    output_path: &PathBuf,
    template_contents: &str,
    data: &serde_json::Map<String, serde_json::Value>,
) {
    //
    let mut reg = handlebars::Handlebars::new();
    reg.set_strict_mode(true);
    reg.register_template_string("zone_xml", template_contents)
        .unwrap();

    let zone_xml = reg.render("zone_xml", data).unwrap();

    // write to file
    let output_prefix = output_path.parent().unwrap();
    let output_base = output_path.file_name().unwrap();
    let outpub_ext = output_path.extension().unwrap();
    std::fs::create_dir_all(output_prefix).unwrap();

    let mut full_path = PathBuf::from(output_prefix);
    let file_name = std::format!(
        "{}_{}.{}",
        output_base.to_str().unwrap(),
        zone,
        outpub_ext.to_str().unwrap()
    );
    full_path.push(file_name);
    std::fs::write(full_path, zone_xml.as_bytes()).unwrap();
}
