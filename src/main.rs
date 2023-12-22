mod open_file;
mod write_file;
mod split_file;
mod file_utils;

use std::env;

use open_file::open_file;
use write_file::write_file;
use split_file::split_file;
use file_utils::file_ext;


fn main () {

    /*
        Run example: 'cargo run -- /path_to_input_file/file.file_extension /path_to_output_file/ file_name
    */

    let args: Vec<_>= env::args().collect();

    let input_file = &args[1];
    let output_dir = &args[2];
    let output_file = &args[3];

    let file_extension = file_ext(input_file);

    let data_in_bytes = open_file(input_file);

    let files = split_file(data_in_bytes, output_dir, output_file, file_extension);

    // write_file(output_file, data_in_bytes);
}
