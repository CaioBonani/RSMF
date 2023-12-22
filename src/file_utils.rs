pub (crate) fn file_ext (input_file: &str) -> &str{

    let (_, file_extension) = input_file.rsplit_once('.').unwrap();    

    return (file_extension)
}