use std::path::PathBuf;

use calamine::{open_workbook, DataType, Range, Reader, Xlsx};

use crate::field_aliase::AliaseMapper;

use super::cell::{CellData, CellType};
use super::excel_rows::{ExcelRow, ExcelRows};
use super::json_rows::{JsonRow, JsonRows};
use super::title::{ExcelTitle, FieldInfo};

const START_ROW: usize = 3_usize; // the 3th row

///
pub fn read_json_rows_from_xlsx(key_name: &str, path: &PathBuf) -> JsonRows {
    //
    let mut json_rows = JsonRows {
        key_2_row_table: hashbrown::HashMap::new(),
        row_table: hashbrown::HashMap::new(),
    };

    // opens a new workbook
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");

    // Read whole worksheet data and provide some statistics
    let sheet_name = "Sheet1";
    let sheet_idx = 0_usize;

    // Open "Sheet1" or first sheet
    if let Ok(range) = workbook.worksheet_range(sheet_name) {
        //
        let total_cells = range.get_size().0 * range.get_size().1;
        let non_empty_cells: usize = range.used_cells().count();
        log::info!(
            "Found {} cells in {}', including {} non empty cells",
            sheet_name,
            total_cells,
            non_empty_cells
        );

        //
        fill_json_rows(key_name, &range, &mut json_rows);
    } else if let Some(range_ret) = workbook.worksheet_range_at(sheet_idx) {
        match range_ret {
            Ok(range) => {
                let sheet_idx_name = sheet_idx.to_string();

                //
                let total_cells = range.get_size().0 * range.get_size().1;
                let non_empty_cells: usize = range.used_cells().count();
                log::info!(
                    "Found {} cells in {}', including {} non empty cells",
                    sheet_idx_name,
                    total_cells,
                    non_empty_cells
                );

                //
                fill_json_rows(key_name, &range, &mut json_rows);
            }
            Err(err) => {
                //
                log::error!(
                    "Open sheet {}/{} failed!!! error: {}!!!",
                    sheet_name,
                    sheet_idx,
                    err
                );
            }
        }
    } else {
        let err_msg = std::format!("Cannot find sheet: {}/{}!!!", sheet_name, sheet_idx);
        log::error!("{}", err_msg);
        std::panic!("{}", err_msg);
    };

    //
    json_rows
}

fn fill_json_rows(key_name: &str, range: &Range<DataType>, json_rows: &mut JsonRows) {
    // aliase
    let aliase_mapper = AliaseMapper::new();
    
    // title
    let mut title: ExcelTitle = read_excel_title(key_name, range).unwrap();

    // data
    let excel_rows = read_excel_rows(&mut title, range);

    // key field info
    let key = key_name.to_owned();
    let key_field_info_opt = title.get_field_info_by_name(&key);
    if let Some(key_field_info) = key_field_info_opt {
        //
        assert_ne!(key_field_info.cell_type, CellType::Unknown);

        // walk excel row cells
        for (row_idx, cells) in &excel_rows.row_table {
            //
            let mut json_row = JsonRow::new();
            fill_json_row(&title, *row_idx, cells, &mut json_row);

            let cell_num = json_row.value_table.len();
            if cell_num > 0 {
                // row is not empty
                let val_of_key_opt = json_row.get_value_as_string(&key);
                if let Some(val_of_key) = val_of_key_opt {
                    // update aliase
                    aliase_mapper.update(&mut json_row.value_table);

                    // collect row
                    json_rows.row_table.insert(*row_idx, json_row);

                    // collect val_of_key
                    json_rows.key_2_row_table.insert(val_of_key, *row_idx);
                } else {
                    let err_msg = std::format!(
                        "[row={}] json row get_value() failed!!! key: {} not found!!!",
                        *row_idx,
                        key
                    );
                    log::error!("{}", err_msg);
                    std::panic!("{}", err_msg);
                }
            } else {
                // force finish when empty row occurred
                log::info!(
                    "\r\n>>>> <<<< [row={}] empty row occurred, break. >>>> <<<<",
                    *row_idx
                );
                break;
            }
        }
    } else {
        let err_msg = std::format!(
            "get_field_info_by_name() failed!!! key: {} not found!!!",
            key
        );
        log::error!("{}", err_msg);
        std::panic!("{}", err_msg);
    }
}

fn fill_json_row(title: &ExcelTitle, _row: u32, cells: &ExcelRow, json_row: &mut JsonRow) {
    // walk cells
    for (col, cell) in &cells.cell_table {
        let field_info_opt = title.get_field_info(*col);
        if let Some(field_info) = field_info_opt {
            // fill json value
            let name = field_info.name.clone();
            let json = cell.to_json();
            json_row.value_table.insert(name, json);
        }
    }
}

fn read_excel_title(key_name: &str, range: &Range<DataType>) -> Option<ExcelTitle> {
    //
    let start_row_idx = (START_ROW - 1) as u32;
    let (height, width) = range.get_size();

    let mut title_row: isize = -1;
    // field name 2 column idx
    let mut column_table = hashbrown::HashMap::<String, u32>::new();
    // clolum idx 2 field info
    let mut field_info_table = hashbrown::HashMap::<u32, FieldInfo>::new();

    // walk row and column
    for row in start_row_idx..height as u32 {
        //
        column_table.clear();
        field_info_table.clear();

        let mut is_title_row = true;
        let mut has_empty_cell = false;

        for col in 0_u32..width as u32 {
            let cell_opt = range.get_value((row, col));
            if let Some(cell) = cell_opt {
                // field title must be string type
                if !cell.is_string() {
                    // ignore row
                    is_title_row = false;
                    break;
                }

                // is comment ?
                let s = cell.as_string().unwrap();
                let s = s.trim();
                if s.is_empty() || s.starts_with("#") || s.starts_with("//") {
                    // ignore row
                    is_title_row = false;
                    break;
                }

                // is empty field name alread found?
                if has_empty_cell && !s.is_empty() {
                    // ignore row -- found empty field name
                    is_title_row = false;
                    break;
                }

                // skip empty cell till to tail
                if s.is_empty() {
                    // mark empty cell found and continue
                    has_empty_cell = true;
                    continue;
                }

                // is it can be converted to string ?
                if s.parse::<i64>().is_ok() {
                    // ignore row
                    is_title_row = false;
                    break;
                }

                // collect
                let mut field_info = FieldInfo::new();
                field_info.column = col;
                field_info.name = s.to_string();
                field_info.desc = "".to_owned();
                field_info.cell_type = CellType::Unknown;

                //
                if key_name == s {
                    field_info.is_key = true;
                } else {
                    field_info.is_key = false;
                }

                field_info_table.insert(col, field_info);

                //
                column_table.insert(s.to_string(), col);
            }
        }

        // found title row?
        if is_title_row {
            title_row = row as isize;
            break;
        }
    }

    let title = ExcelTitle {
        title_row,
        column_table,
        field_info_table,
    };

    Some(title)
}

fn read_excel_rows(title: &mut ExcelTitle, range: &Range<DataType>) -> ExcelRows {
    //
    assert!(title.title_row >= 0);
    let data_row_idx = (title.title_row + 1) as u32;
    let column_max = title.column_table.len();
    let (height, _width) = range.get_size();

    // row idx 2 row cells
    let mut row_table = std::collections::BTreeMap::<u32, ExcelRow>::new();
    // clolum idx 2 cell_data
    let mut cell_table = std::collections::BTreeMap::<u32, CellData>::new();

    // walk row and column
    for row in data_row_idx..height as u32 {
        //
        let mut is_row_empty = true;

        //
        for col in 0_u32..column_max as u32 {
            let mut data_opt: Option<CellData> = None;

            //
            let cell_opt = range.get_value((row as u32, col as u32));
            if let Some(cell) = cell_opt {
                match cell {
                    DataType::Int(n) => {
                        //
                        let mut data = CellData::new();
                        data.row = row;
                        data.col = col;
                        data.cell_type = CellType::Integer;
                        data.integer_val = *n;

                        //
                        title.sync_cell_type(&mut data);
                        title.check_cell_type(row, &data);

                        // collect
                        data_opt = Some(data);
                    }
                    DataType::Float(f) => {
                        //
                        let mut data = CellData::new();
                        data.row = row;
                        data.col = col;
                        data.cell_type = CellType::Float;
                        data.float_val = *f;

                        //
                        title.sync_cell_type(&mut data);
                        title.check_cell_type(row, &data);

                        // collect
                        data_opt = Some(data);
                    }
                    DataType::String(s) => {
                        //
                        let mut data = CellData::new();
                        data.row = row;
                        data.col = col;
                        data.cell_type = CellType::String;
                        data.string_val = s.to_owned();

                        //
                        title.sync_cell_type(&mut data);
                        title.check_cell_type(row, &data);

                        // collect
                        data_opt = Some(data);
                    }
                    DataType::Bool(b) => {
                        //
                        let mut data = CellData::new();
                        data.row = row;
                        data.col = col;
                        data.cell_type = CellType::Integer;
                        if *b {
                            data.integer_val = 1_i64;
                        } else {
                            data.integer_val = 0_i64;
                        }

                        //
                        title.sync_cell_type(&mut data);
                        title.check_cell_type(row, &data);

                        // collect
                        data_opt = Some(data);
                    }
                    DataType::DateTime(f) => {
                        //
                        let mut data = CellData::new();
                        data.row = row;
                        data.col = col;
                        data.cell_type = CellType::Float;
                        data.float_val = *f;

                        //
                        title.sync_cell_type(&mut data);
                        title.check_cell_type(row, &data);

                        // collect
                        data_opt = Some(data);
                    }
                    DataType::Duration(f) => {
                        //
                        let mut data = CellData::new();
                        data.row = row;
                        data.col = col;
                        data.cell_type = CellType::Float;
                        data.float_val = *f;

                        //
                        title.sync_cell_type(&mut data);
                        title.check_cell_type(row, &data);

                        // collect
                        data_opt = Some(data);
                    }
                    // Date, Time or DateTime in ISO 8601
                    DataType::DateTimeIso(s) => {
                        //
                        let mut data = CellData::new();
                        data.row = row;
                        data.col = col;
                        data.cell_type = CellType::String;
                        data.string_val = s.to_owned();

                        //
                        title.sync_cell_type(&mut data);
                        title.check_cell_type(row, &data);

                        // collect
                        data_opt = Some(data);
                    }
                    // Duration in ISO 8601
                    DataType::DurationIso(s) => {
                        //
                        let mut data = CellData::new();
                        data.row = row;
                        data.col = col;
                        data.cell_type = CellType::String;
                        data.string_val = s.to_owned();

                        //
                        title.sync_cell_type(&mut data);
                        title.check_cell_type(row, &data);

                        // collect
                        data_opt = Some(data);
                    }
                    // Error
                    DataType::Error(err) => {
                        // skip
                        log::error!("cell({},{}) error: {:?}!!!", row, col, err);
                    }
                    // Empty cell
                    DataType::Empty => {
                        // log empty
                        //log::info!("cell({},{}) is empty.", row, col);
                    }
                }

                //
                if let Some(data) = data_opt {
                    // collect
                    cell_table.insert(col, data);
                    is_row_empty = false;
                } else {
                    // log null
                    // log::info!(
                    //     "\r\n**** **** null cell({},{}) **** ****",
                    //     row,
                    //     col
                    // );
                    cell_table.insert(col, CellData::new());
                }
            }
        }

        // collect and clear last row at beginning
        if !is_row_empty {
            assert!(cell_table.len() > 0);
            row_table.insert(row, ExcelRow { cell_table });

            // new row cells
            cell_table = std::collections::BTreeMap::<u32, CellData>::new();
        } else {
            // force finish when empty row occurred
            log::info!(
                "\r\n>>>> <<<< [row={}] empty row occurred, break. >>>> <<<<",
                row
            );
            break;
        }
    }

    ExcelRows { row_table }
}
