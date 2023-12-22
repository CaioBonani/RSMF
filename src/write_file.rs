use std::fs;

pub (crate) fn write_file (output_file: &str, data_bytes: Vec<u8>) {

    fs::write(output_file, data_bytes).expect("Error creating the file.");
}