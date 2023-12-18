use std::{fs::File, io::Read};

pub fn read_txt_to_vec(filename: &str) -> Vec<String> {
    let mut file_content = String::new();
    let mut data_file = File::open(filename).unwrap();
    data_file.read_to_string(&mut file_content).unwrap();
    let result: Vec<String> = file_content.split('\n').map(String::from).collect();
    return result;
}
