use std::fs;

pub (crate) fn split_file (data_bytes: Vec<u8>, output_dir: &str, output_file: &str, file_extension: &str) {

    const FILE_MAX_SIZE: usize = 25000000;

    let mut i = 0;

    for part in data_bytes.chunks(FILE_MAX_SIZE) {

        fs::write(output_dir.to_owned() + output_file + i.to_string().as_str() + '.'.to_string().as_str() + file_extension, part).expect("Error creating files.");
        i = i + 1;
    }    
}