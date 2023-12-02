use std::{fs::File, io::Read};

fn main() {
    println!("Advent of Code 2023 Answers!");
    aoc_1_a();
}

fn aoc_1_a() {
    let input = read_txt_to_vec("inputs/1-b.txt");
    let output: u64 = input.iter().map(|x| str_sum(x)).sum();
    println!("Result for first part: {}", output);
}

fn aoc_1_b() {
    let input = read_txt_to_vec("inputs/1-a.txt");
    let output = todo!();
    println!("Result for second part: {}", output);
}

fn str_sum(text: &str) -> u64 {
    let mut result = 0;
    let text_bytes = text.as_bytes();

    for &byte in text_bytes {
        if byte.is_ascii_digit() {
            result += u64::from(byte - b'0') * 10;
            break;
        }
    }
    for &byte in text_bytes.iter().rev() {
        if byte.is_ascii_digit() {
            result += u64::from(byte - b'0');
            break;
        }
    }
    return result;
}

fn str_sum_plus(text: &str) -> u64 {
    let list_of_digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    todo!();
}

fn read_txt_to_vec(filename: &str) -> Vec<String> {
    let mut file_content = String::new();
    let mut data_file = File::open(filename).unwrap();
    data_file.read_to_string(&mut file_content).unwrap();
    let result: Vec<String> = file_content.split('\n').map(String::from).collect();
    return result;
}
