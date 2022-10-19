use csv::{ReaderBuilder};
use ndarray::{Array2};
use ndarray_csv::{Array2Reader};
use std::error::Error;
use std::fs::File;

pub fn import_csv(file_name: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_name)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);
    let array_read: Array2<u64> = reader.deserialize_array2((42000, 785))?;
    
    println!("{:?}", array_read);

    Ok(())
}