use std::fs::File;
use std::io::{BufReader, Read};

/// look for word XMAS in a matrix of characters and return the number of times it appears
/// it can appear in any direction (horizontal, vertical, diagonal) and in Upper case.
/// totals 8 directions to look for the word


fn get_input() -> Vec<Vec<char>> {
    let file = File::open("inputs/input_day04.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();
    let matrix = buffer.lines().map(|l| l.chars().collect()).collect();
    matrix
}

fn main() -> i32 {
    let matrix = &get_input();
    let mut number_of_matches = 0;

    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == 'X' {
                check_word(matrix, row, col, &mut number_of_matches);
            }
        }
    }
    number_of_matches
}

fn check_word(matrix: &Vec<Vec<char>>, row: usize, col: usize, number_of_matches: &mut i32) {
    let directions = vec![(0, 1), (1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1), (-1, 0), (0, -1)];
    let word = vec!['X', 'M', 'A', 'S'];
    for (dx, dy) in directions {
        let mut found = true;
        for i in 0..word.len() {
            let new_row = row as i32 + i as i32 * dx;
            let new_col = col as i32 + i as i32 * dy;
            if new_row < 0 || new_row >= matrix.len() as i32 || new_col < 0 || new_col >= matrix[0].len() as i32 {
                found = false;
                break;
            }
            if matrix[new_row as usize][new_col as usize] != word[i] {
                found = false;
                break;
            }
        }
        if found {
            *number_of_matches += 1;
        }
    }
}


#[test]
fn part2(){
    let matrix = &get_input();
    let mut number_of_matches = 0;

    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            if matrix[row][col] == 'A' {
                check_crossed_mas(matrix, row, col, &mut number_of_matches);
            }
        }
    }
    println!("Number of matches: {}", number_of_matches);
}

/// now we need to look for MAS in a crossed pattern in the matrix starting with A in the middle
fn check_crossed_mas(matrix: &Vec<Vec<char>>, row: usize, col: usize, number_of_matches: &mut i32) {

    let mut found_left_diag = false;
    let mut found_right_diag = false;

    let left_bottom = (row as i32 + 1, col as i32 - 1);
    let right_top = (row as i32 - 1 , col as i32 + 1);
    let left_top = (row as i32 - 1 , col as i32 - 1);
    let right_bottom = (row as i32 + 1 , col as i32 + 1);

    let positions = [left_bottom, right_top, right_bottom, left_top];
    for &(row, col) in &positions {
        if row < 0 || col < 0 || row >= matrix.len() as i32 || col >= matrix[0].len() as i32 {
            found_left_diag = false;
            found_right_diag = false;
            return;
        }
    }

    found_left_diag = (matrix[left_bottom.0 as usize][left_bottom.1 as usize] == 'M' && matrix[right_top.0 as usize][right_top.1 as usize] == 'S') ||
        (matrix[left_bottom.0 as usize][left_bottom.1 as usize] == 'S' && matrix[right_top.0 as usize][right_top.1 as usize] == 'M');

    found_right_diag = (matrix[left_top.0 as usize][left_top.1 as usize] == 'M' && matrix[right_bottom.0 as usize][right_bottom.1 as usize] == 'S') ||
        (matrix[left_top.0 as usize][left_top.1 as usize] == 'S' && matrix[right_bottom.0 as usize][right_bottom.1 as usize] == 'M');

    if found_left_diag && found_right_diag {
        *number_of_matches += 1;
    }
}