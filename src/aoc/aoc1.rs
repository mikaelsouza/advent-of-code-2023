use crate::aoc::utils;

pub fn main() {
    let input = utils::read_txt_to_vec("inputs/1-b.txt");
    let output: u64 = input.iter().map(|x| str_sum(x)).sum();
    println!("Result for first part: {}", output);
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
