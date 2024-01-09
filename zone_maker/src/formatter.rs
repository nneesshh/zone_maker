///
pub trait FormatToFile {
    ///
    fn format(&self) -> std::path::PathBuf;
}

///
pub mod ini_formatter;

///
pub mod mysql_formatter;

///
pub mod excel_formatter;
