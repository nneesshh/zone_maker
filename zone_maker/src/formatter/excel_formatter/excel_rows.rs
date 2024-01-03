use super::excel_cell_data::CellData;

///
pub struct ExcelRows {
    pub row_table: std::collections::BTreeMap<u32, ExcelRow>, // row idx 2 row cells
}

impl ExcelRows {
    ///
    #[allow(dead_code)]
    pub fn get_row(&self, row: u32) -> Option<&ExcelRow> {
        self.row_table.get(&row)
    }
}

///
pub struct ExcelRow {
    pub cell_table: std::collections::BTreeMap<u32, CellData>, // column idx 2 cell
}

impl ExcelRow {
    ///
    #[allow(dead_code)]
    pub fn get_cell_data(&self, col: u32) -> Option<&CellData> {
        self.cell_table.get(&col)
    }
}
