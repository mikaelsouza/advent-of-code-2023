use crate::aoc::utils;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct Value {
    value: i32,
    starting_point: Point,
}

pub fn main() {
    let input = utils::read_txt_to_vec("inputs/3-b.txt");
    let matrix = convert_to_2d_matrix(input);
    let symbols = find_symbols(&matrix);
    let num_points = get_number_points(&matrix, symbols);
    let values = get_num_value_from_point(&matrix, num_points);
    let values = remove_duplicates(values);
    let final_result = sum_values(values);
    println!("Result: {:?}", final_result);
}

fn sum_values(values: Vec<Value>) -> i32 {
    let mut result = 0;
    for value in values {
        result += value.value;
    }
    result
}

fn remove_duplicates(values: Vec<Value>) -> Vec<Value> {
    let mut result: Vec<Value> = vec![];
    for value in values {
        if !result.contains(&value) {
            result.push(value);
        }
    }
    result
}
fn convert_to_2d_matrix(values: Vec<String>) -> Vec<Vec<char>> {
    let mut matrix: Vec<Vec<char>> = vec![];
    for line in values {
        matrix.push(line.chars().collect());
    }
    matrix
}

fn find_symbols(matrix: &Vec<Vec<char>>) -> Vec<Point> {
    let mut symbol_pos: Vec<Point> = vec![];
    for (i, row) in matrix.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if !col.is_numeric() && *col != '.' {
                symbol_pos.push(Point {
                    x: i as i32,
                    y: j as i32,
                });
            }
        }
    }
    symbol_pos
}

fn check_surroundings(matrix: &Vec<Vec<char>>, point: Point) -> Vec<Point> {
    let surrondings: Vec<(i32, i32)> = vec![
        (0, 1),
        (1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
        (0, -1),
        (-1, 0),
    ];

    let max_i = matrix.len() as i32;
    let max_j = matrix[0].len() as i32;

    let mut results: Vec<Point> = vec![];

    for option in surrondings {
        let i = point.x + option.0;
        let j = point.y + option.1;
        if i < 0 || i >= max_i || j < 0 || j >= max_j {
            continue;
        }
        if matrix[i as usize][j as usize].is_numeric() {
            results.push(Point { x: i, y: j });
        }
    }
    results
}

fn get_number_points(matrix: &Vec<Vec<char>>, points: Vec<Point>) -> Vec<Point> {
    let mut numbers: Vec<Point> = vec![];
    for point in points {
        let result = check_surroundings(&matrix, point);
        for r in result {
            numbers.push(r);
        }
    }
    numbers
}

fn get_num_value_from_point(matrix: &Vec<Vec<char>>, points: Vec<Point>) -> Vec<Value> {
    let mut values: Vec<Value> = vec![];

    for point in points {
        let i = point.x;
        let mut j = point.y;
        while j >= 0 && matrix[i as usize][j as usize].is_numeric() {
            j -= 1;
        }
        j += 1;
        let starting_point = Point { x: i, y: j };
        let mut value: i32 = 0;

        while (j as usize) < matrix[0].len() && matrix[i as usize][j as usize].is_numeric() {
            let r = matrix[i as usize][j as usize].to_digit(10).unwrap() as i32;
            value = value * 10 + r;
            j += 1;
        }
        values.push(Value {
            value,
            starting_point,
        })
    }
    return values;
}

#[cfg(test)]
mod tests {
    use super::{convert_to_2d_matrix, find_symbols, Point};
    #[test]
    fn test_convert_to_2d_matrix() {
        let input = vec![String::from("abc"), String::from("def")];
        let result = convert_to_2d_matrix(input);
        assert_eq!(result[0][0], 'a');
        assert_eq!(result[1][1], 'e');
        assert_eq!(result, vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]);
        assert_ne!(result[0][0], 'b');
    }

    #[test]
    fn test_find_symbols_1() {
        let input = vec![vec!['1', '2', '.'], vec!['.', '3', '.']];
        let result = find_symbols(&input);
        println!("{:?}", result);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_find_symbols_2() {
        let input = vec![vec!['1', '#', '.'], vec!['2', '$', '=']];
        let result = find_symbols(&input);
        println!("{:?}", result);
        assert_eq!(
            result,
            vec![
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
                Point { x: 1, y: 2 },
            ]
        );
    }
}
