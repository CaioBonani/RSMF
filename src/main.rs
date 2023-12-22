mod open_file;
mod write_file;

use std::env;

use open_file::open_file;
use write_file::write_file;


fn main() {

    /*
        Run example: 'cargo run -- /dest_to_input_file/file.file_extension /dest_to_output_file/file.file_extension
    */

    let args: Vec<_>= env::args().collect();

    let input_file = &args[1];
    let output_file = &args[2];

    let data_in_bytes = open_file(input_file);

    write_file(output_file, data_in_bytes);
}
