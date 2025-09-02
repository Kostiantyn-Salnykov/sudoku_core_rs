use serde_json::Value;
use std::fs::read_to_string;

pub fn load_json(filename: &str) -> Vec<Vec<Option<u8>>> {
    let data =
        read_to_string(filename).unwrap_or_else(|_| panic!("Failed to read the '{filename}'."));
    let json: Value = serde_json::from_str(&data).expect("Invalid JSON.");

    json.as_array()
        .expect("Expected array of arrays.")
        .iter()
        .map(|row| {
            row.as_array()
                .expect("Expected array.")
                .iter()
                .map(|cell| {
                    if cell.is_null() {
                        None
                    } else {
                        Some(cell.as_u64().expect("Expected positive number.") as u8)
                    }
                })
                .collect()
        })
        .collect()
}
