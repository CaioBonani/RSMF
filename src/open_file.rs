use std::fs;
use std::io::Read;

pub (crate) fn open_file (input_file: &str) -> Vec<u8>{

    let mut file = fs::File::open(input_file).unwrap();
    let mut buffer: Vec<u8> = vec![];

    file.read_to_end(&mut buffer).unwrap();

    return (buffer)
}

