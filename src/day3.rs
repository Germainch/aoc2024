use std::fs::File;
use std::io::{BufReader, Read};
use std::result;
use regex::Regex;

#[test]
fn file_to_string(){
    let file = File::open("inputs/input_day03.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();
    let result = regex_match_mul(&buffer);
    println!("Result: {}", result);
}

fn regex_match_mul(buffer: &String) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut x = 0;
    let mut y = 0;
    let mut res = 0;

    for (_, [first,second]) in re.captures_iter(&buffer).map(|c| c.extract()) {
        x = first.parse::<i32>().unwrap();
        y = second.parse::<i32>().unwrap();
        res += x * y;
    }
    res
}

#[test]
fn file_to_string_do(){
    let start_time = std::time::Instant::now();
    let file = File::open("inputs/input_day03.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();
    let result = regex_match_mul_conditioned(&buffer);
    println!("Result: {}", result);
    println!("Elapsed time: {:?}", start_time.elapsed());
}
fn regex_match_mul_conditioned(buffer: &String) -> i32 {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();
    let mut x = 0;
    let mut y = 0;
    let mut res = 0;
    let mut enabled = true;

    for (c) in re.captures_iter(&buffer) {

        if re_do.is_match(&c[0]) {
            enabled = true;
        } else if re_dont.is_match(&c[0]) {
            enabled = false;
        } else {
            if enabled {
                x = c[1].parse::<i32>().unwrap();
                y = c[2].parse::<i32>().unwrap();
                res += x * y;
            }
        }
    }
    res
}