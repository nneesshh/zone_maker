use crate::utils::misc::float_to_string;

use super::cell::{CellData, CellType};

///
#[derive(Debug)]
pub struct FieldInfo {
    pub column: u32,
    pub name: String,
    pub desc: String,
    pub cell_type: CellType,

    pub is_key: bool,
}

impl FieldInfo {
    ///
    pub fn new() -> Self {
        Self {
            column: 0,
            name: "".to_owned(),
            desc: "".to_owned(),
            cell_type: CellType::Unknown,

            is_key: false,
        }
    }
}

///
pub struct ExcelTitle {
    pub title_row: isize,
    pub column_table: hashbrown::HashMap<String, u32>, // name 2 column idx
    pub field_info_table: hashbrown::HashMap<u32, FieldInfo>, // column idx 2 info
}

impl ExcelTitle {
    ///
    #[allow(dead_code)]
    pub fn get_field_info_mut(&mut self, col: u32) -> Option<&mut FieldInfo> {
        self.field_info_table.get_mut(&col)
    }

    ///
    #[allow(dead_code)]
    pub fn get_field_info(&self, col: u32) -> Option<&FieldInfo> {
        self.field_info_table.get(&col)
    }

    ///
    #[allow(dead_code)]
    pub fn get_field_info_by_name_mut(&mut self, name: &str) -> Option<&mut FieldInfo> {
        let column_opt = self.column_table.get(name);
        if let Some(col) = column_opt {
            self.field_info_table.get_mut(col)
        } else {
            None
        }
    }

    ///
    #[allow(dead_code)]
    pub fn get_field_info_by_name(&self, name: &str) -> Option<&FieldInfo> {
        let column_opt = self.column_table.get(name);
        if let Some(col) = column_opt {
            self.field_info_table.get(col)
        } else {
            None
        }
    }

    /// Solve the conflict between field info and cell data
    pub fn sync_cell_type(&mut self, data: &mut CellData) {
        let col = data.col;
        let field_info_opt = self.get_field_info_mut(col);
        if let Some(field_info) = field_info_opt {
            if field_info.cell_type == CellType::Unknown {
                // update
                field_info.cell_type = data.cell_type;
            } else if field_info.cell_type == CellType::String {
                // convert Integer and Float to String
                if data.cell_type == CellType::Integer {
                    //
                    data.cell_type = CellType::String;
                    data.string_val = data.integer_val.to_string();
                } else if data.cell_type == CellType::Float {
                    //
                    data.cell_type = CellType::String;
                    data.string_val = float_to_string(data.float_val);
                }
            }
        }
    }

    ///
    pub fn check_cell_type(&self, row: u32, data: &CellData) -> bool {
        //
        let col = data.col;
        let field_info_opt = self.get_field_info(col);
        if let Some(field_info) = field_info_opt {
            match field_info.cell_type {
                CellType::Unknown => {
                    // error
                    let err_msg = std::format!(
                        "\r\n[row={}] check cell({:?}) failed!!! \r\n========\r\n {:?}!!!",
                        row,
                        data,
                        field_info
                    );
                    std::panic!("{}", err_msg);
                }
                CellType::Null => {
                    // allow Null only
                    let err_msg = std::format!(
                        "\r\n[row={}] cell({:?}) need to be null!!! \r\n========\r\n {:?}!!!",
                        row,
                        data,
                        field_info
                    );
                    std::panic!("{}", err_msg);
                }
                CellType::String => {
                    // allow String only
                    if data.cell_type != CellType::String {
                        let err_msg = std::format!(
                            "\r\n[row={}] cell({:?}) type mismatch!!! \r\n========\r\n {:?}!!!",
                            row,
                            data,
                            field_info
                        );
                        std::panic!("{}", err_msg);
                    } else {
                        return true;
                    }
                }
                CellType::Integer | CellType::Float => {
                    // allow Integer, Float
                    if data.cell_type != CellType::Integer && data.cell_type != CellType::Float {
                        let err_msg = std::format!(
                            "\r\n[row={}] cell({:?}) type mismatch!!! \r\n========\r\n {:?}!!!",
                            row,
                            data,
                            field_info
                        );
                        std::panic!("{}", err_msg);
                    } else {
                        return true;
                    }
                }
            }
        }
        false
    }
}
