use std::fs;
use std::io::Read;

pub (crate) fn merge_files (files: Vec<String>, output_dir: &str, output_file: &str, file_extension: &str) {

    let mut splited_data: Vec<u8> = vec![];

    for file in files {

        let mut buffer: Vec<u8> = vec![];
        let mut file_op = fs::File::open(file).unwrap();
        file_op.read_to_end(&mut buffer).unwrap();

        splited_data.append(&mut buffer);
    }

    fs::write(output_dir.to_owned() + output_file + '.'.to_string().as_str() + file_extension, splited_data).expect("Error creating the file.");

}