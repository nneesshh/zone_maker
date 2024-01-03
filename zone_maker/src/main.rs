use std::path::PathBuf;

use clap::Parser;

mod db_access;
mod sqls;
use db_access::MySqlAddr;

//mod field_mapper;
mod formatter;
mod template_writer;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, verbatim_doc_comment, long_about = None, disable_help_flag = true, arg_required_else_help = true)]
struct ZoneMaker {
    #[command(subcommand)]
    commands: Commands,

    #[clap(long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,

    /// File path for log config
    #[arg(
        short = 'l',
        long,
        default_value = "res/log4rs.yaml",
        global = true,
        verbatim_doc_comment
    )]
    log_path: String,

    /// Zone id
    #[arg(
        short = 'z',
        long,
        default_value = "0",
        value_name = "ZONE_ID(0 for all)",
        global = true,
        verbatim_doc_comment
    )]
    zone_id: String,

    /// File path for log config
    #[arg(
        short = 'o',
        long,
        default_value = "out/dragon.xml",
        value_name = "OUTPUT_FILE_PATH",
        global = true,
        verbatim_doc_comment
    )]
    output_path: String,

    /// File path for template
    #[arg(
        index = 1,
        default_value = "templates/default.tpl",
        value_name = "TPL_FILE_PATH",
        global = true,
        verbatim_doc_comment
    )]
    template_path: String,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    INI(Box<Toml>),
    DB(Box<MySqlAddr>),
    EXCEL(Box<Excel>),
}

///
#[derive(clap::Args, Debug)]
pub struct Toml {
    #[arg(
        short = 'i',
        long,
        default_value = "res/zone.ini",
        value_name = "INI_FILE_PATH",
        verbatim_doc_comment
    )]
    pub ini_path: String,
}

///
#[derive(clap::Args, Debug)]
pub struct Excel {
    #[arg(
        short = 'i',
        long,
        default_value = "res/default.xlsx",
        value_name = "XLSX_FILE_PATH",
        verbatim_doc_comment
    )]
    pub xlsx_path: String,
}

fn main() {
    //
    let args = ZoneMaker::parse();

    //
    let _ = log4rs::init_file(args.log_path, Default::default());

    //
    let zone_id = args.zone_id.as_str();
    let template_file_path = PathBuf::from(&args.template_path);
    let output_file_path = PathBuf::from(&args.output_path);

    //
    match args.commands {
        Commands::INI(toml) => {
            log::info!(
                "zone maker start to generate config from ini: {:?} ...",
                toml
            );

            // ini
            let ini_file_path = PathBuf::from(toml.ini_path);
            let mut formatter = formatter::toml_formatter::TomlFormatter::new(
                ini_file_path,
                template_file_path,
                output_file_path,
            );
            formatter.format();

            log::info!("zone maker generate config to ({:?})", args.output_path);
        }

        Commands::DB(mysql_addr) => {
            log::info!(
                "zone maker start to generate config from db: {:?} ...",
                mysql_addr
            );

            // db
            let mut formatter = formatter::mysql_formatter::MySqlFormatter::new(
                *mysql_addr,
                template_file_path,
                output_file_path,
            );
            formatter.format();

            log::info!("zone maker generate config to ({:?})", args.output_path);
        }

        Commands::EXCEL(excel) => {
            log::info!(
                "zone maker start to generate config from excel: {:?} ...",
                excel
            );

            // xlsx
            let xlsx_file_path = PathBuf::from(excel.xlsx_path);
            let mut formatter = formatter::excel_formatter::ExcelFormatter::new(
                zone_id,
                xlsx_file_path,
                template_file_path,
                output_file_path,
            );
            formatter.format();

            log::info!("zone maker generate config to ({:?})", args.output_path);
        }
    }
}
