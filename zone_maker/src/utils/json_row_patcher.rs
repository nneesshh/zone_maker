use serde_json::Value as Json;

use super::json_rows::JsonRows;

///
pub struct JsonRowPatcher {
    ///
    pub patch_table: serde_json::Map<String, Json>, // zone_id 2 patch value
}

impl JsonRowPatcher {
    ///
    pub fn new(j_patch: &str) -> Self {
        //
        let mut j: Json = serde_json::from_str(j_patch).unwrap();
        let patch_table = j.as_object_mut().unwrap();

        Self {
            patch_table: patch_table.clone(),
        }
    }

    ///
    pub fn check_patchable(&self, key_name: &str, val_of_key: &Json) -> bool {
        let patch_val_opt = self.patch_table.get(key_name);
        if let Some(patch_val) = patch_val_opt {
            if patch_val == val_of_key {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    ///
    pub fn update(&self, key_name: &str, json_rows: &mut JsonRows) {
        //
        for (_row_idx, json_row) in &mut json_rows.row_table {
            //
            let val_of_key_opt = json_row.get_value(key_name);
            if let Some(val_of_key) = val_of_key_opt {
                //
                let patchable = self.check_patchable(key_name, val_of_key);
                if patchable {
                    for (key, val) in &self.patch_table {
                        json_row.value_table.insert(key.clone(), val.clone());
                    }
                }
            }
        }
    }
}
