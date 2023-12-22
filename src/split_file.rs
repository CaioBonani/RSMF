use std::{fs, vec};

pub (crate) fn split_file (data_bytes: Vec<u8>, output_dir: &str, output_file: &str) -> Vec<String>{

    const FILE_MAX_SIZE: usize = 25000000;

    let mut i = 0;

    let mut files: Vec<String> = vec![];

    for part in data_bytes.chunks(FILE_MAX_SIZE) {

        let path_plus_file =  output_dir.to_owned() + output_file + i.to_string().as_str();
        fs::write(&path_plus_file, part).expect("Error creating files.");
        i = i + 1;
        files.push(path_plus_file);
    } 

    return (files);
}