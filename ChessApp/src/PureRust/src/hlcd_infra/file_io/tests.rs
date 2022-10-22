use super::*;

fn get_file_io() -> impl FileIOInterface {
    FileIO::new()
}

#[test]
fn test_get_current_directory() {
    let io = get_file_io();
    assert!(io.get_current_directory().unwrap().len() > 0);
}