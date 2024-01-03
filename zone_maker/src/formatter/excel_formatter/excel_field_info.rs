use super::excel_cell_data::CellType;

///
#[derive(Debug)]
pub struct FieldInfo {
    pub column: u32,
    pub name: String,
    pub desc: String,
    pub cell_type: CellType,

    pub is_ascii: bool,
}

impl FieldInfo {
    ///
    pub fn new() -> Self {
        Self {
            column: 0,
            name: "".to_owned(),
            desc: "".to_owned(),
            cell_type: CellType::Unknown,

            is_ascii: false,
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
    pub fn get_field_info_by_name_mut(&mut self, name:&str) -> Option<&mut FieldInfo> {
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
}
