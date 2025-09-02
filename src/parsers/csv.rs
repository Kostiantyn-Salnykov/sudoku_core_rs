use std::fs;
use tracing::error;

pub fn load_csv(filename: &str) -> Vec<Vec<Option<u8>>> {
    let file = fs::File::open(filename).unwrap();
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    let mut data: Vec<Vec<Option<u8>>> = Vec::with_capacity(9);

    for result in csv_reader.records() {
        match result {
            Ok(record) => {
                let parsed_record: Vec<Option<u8>> = record
                    .iter()
                    .map(|field| {
                        let value = field.trim(); // Remove leading and trailing whitespace.
                        if value.is_empty()
                            || value.eq_ignore_ascii_case("null")
                            || value == "*"
                            || value == "0"
                        {
                            None
                        } else {
                            value.parse::<u8>().ok()
                        }
                    })
                    .collect();

                data.push(parsed_record);
            }
            Err(error) => {
                error!("Failed to parse record: {}.", error);
            }
        }
    }

    data
}
