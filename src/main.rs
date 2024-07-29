use csv::ReaderBuilder;
use postcard;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
struct DriverData {
    driver_number: u8,
    led_num: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the CSV file
    let mut rdr = ReaderBuilder::new().from_path("all_data.csv")?;
    let mut driver_data = Vec::new();

    for result in rdr.deserialize() {
        let record: DriverData = result?;
        driver_data.push(record);
    }

    // Serialize to JSON and write to file
    let json_data = serde_json::to_string(&driver_data)?;
    let mut json_file = File::create("all_data.json")?;
    json_file.write_all(json_data.as_bytes())?;

    // Serialize to binary using postcard and write to file
    let bin_data = postcard::to_allocvec(&driver_data)?;
    let mut bin_file = File::create("all_data.bin")?;
    bin_file.write_all(&bin_data)?;

    Ok(())
}
