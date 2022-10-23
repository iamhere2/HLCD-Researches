use super::*;

fn get_file_io() -> impl FileIOInterface {
    FileIO::new()
}

#[test]
fn test_get_current_directory() {
    let io = get_file_io();
    assert!(io.get_current_directory().unwrap().len() > 0);
}

#[test]
fn test_list_read_write_delete_cyle() {
    let io = get_file_io();
    let cur_dir = io.get_current_directory().unwrap();
    
    let tmp_file = "__test_tmp_file.txt";
    let tmp_file_pattern = "__test_tmp_fil?.*";
    let content = "Test Content ЯЫЁЖ";

    assert_eq!(io.list_files(cur_dir.as_str(), tmp_file_pattern).unwrap(), Vec::<String>::new());
    assert_eq!(io.write_file(tmp_file, content).unwrap(), ());
    assert_eq!(io.list_files(cur_dir.as_str(), tmp_file_pattern).unwrap(), vec![tmp_file]);
    assert_eq!(io.read_file(tmp_file).unwrap(), content);
    assert_eq!(io.delete_file(tmp_file).unwrap(), ());
    assert_eq!(io.list_files(cur_dir.as_str(), tmp_file_pattern).unwrap(), Vec::<String>::new());
}