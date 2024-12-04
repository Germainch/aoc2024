use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};





#[test]
fn day02part1(){
    let input = File::open("inputs/input_day02.txt").unwrap();
    let mut reader = BufReader::new(input);
    let mut buffer = String::new();
    let mut safe_reports_number = 0;

    while reader.read_line(&mut buffer).unwrap() != 0 {
        safe_reports_number += process_line(&buffer);
        buffer.clear();
    }
    println!("{safe_reports_number}");
}

fn process_line(line: &String) -> i32 {
    let vec = line.split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    for i in 0..vec.len() {
        if i == 0 {
            continue;
        }
        let diff = (vec[i] - vec[i-1]).abs();
        if  diff > 3 || diff < 1  {
            return 0;
        }
    }
    if vec.is_sorted() || vec.is_sorted_by(|x, x1| { x >= x1 }) {
        return 1;
    }
    0
}


#[test]
fn day2part2(){
    let input = File::open("inputs/input_day02p2.txt").unwrap();
    let mut reader = BufReader::new(input);
    let mut buffer = String::new();
    let mut safe_reports_number = 0;

    while reader.read_line(&mut buffer).unwrap() != 0 {
        if process_vec(line_to_vec(&buffer)) {
            safe_reports_number += 1;
        }
        buffer.clear();
    }

    println!("{safe_reports_number}");
}


fn line_to_vec(line: &String) -> Vec<i32> {
    line.split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}


fn check_vec(vec: &Vec<i32>) -> bool {
    let order =  vec.is_sorted() || vec.is_sorted_by(|x, x1| { x >= x1 });
    let diff = vec.windows(2).all(|w| (w[1] - w[0]).abs() <= 3 && (w[1] - w[0]).abs() >= 1);
    order && diff
}

fn process_vec(vec: Vec<i32>) -> bool {
    // guards
    if vec.len() == 0 { return false; }
    if vec.len() == 1 || check_vec(&vec) { return true; }

    for i in 0..vec.len() {
        let mut clone = vec.clone();
        clone.remove(i);
        if check_vec(&clone) {
            return true;
        }
    }
    false
}