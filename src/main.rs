use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use lazy_static::lazy_static;

fn main() {

    let mut location_ids: Vec<i32> = Vec::new();
    let mut values : Vec<i32> = Vec::new();
    let input = File::open("inputs/input.txt").unwrap();
    let mut reader = BufReader::new(input);
    let mut buffer = String::new();


    while reader.read_line(&mut buffer).unwrap() != 0 {
        let result = buffer.split_whitespace().collect::<Vec<&str>>();
        println!("000");
        location_ids.push(result[0].parse::<i32>().unwrap());
        values.push(result[1].parse::<i32>().unwrap());
        buffer.clear();
    }

    location_ids.sort();
    values.sort();
    let mut sum = 0;

    for i in 0..location_ids.len() {
        sum += (values[i] - location_ids[i]).abs();
    }
    println!("{sum}");
}


#[test]
fn day01part2() {
    let mut similarity_score = 0;

    let mut location_ids: Vec<i32> = Vec::new();
    let mut values : Vec<i32> = Vec::new();
    let input = File::open("inputs/input.txt").unwrap();
    let mut reader = BufReader::new(input);
    let mut buffer = String::new();

    while reader.read_line(&mut buffer).unwrap() != 0 {
        let result = buffer.split_whitespace().collect::<Vec<&str>>();
        location_ids.push(result[0].parse::<i32>().unwrap());
        values.push(result[1].parse::<i32>().unwrap());
        buffer.clear();
    }


    let mut occurence_map: HashMap<i32,i32> = HashMap::new();
    for id in location_ids {
        if occurence_map.contains_key(&id) {
            similarity_score += id * occurence_map.get(&id).unwrap();
        }
        else {
            let mut occurences = 0;
            for value in values.iter_mut() {
                if id == *value {
                    occurences += 1;
                }
            }
            occurence_map.insert(id, occurences);
            similarity_score += id * occurences;
        }
    }
    println!("{similarity_score}");
}

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
        safe_reports_number += process_line2(&buffer);
        buffer.clear();
    }

    println!("{safe_reports_number}");
}

fn process_line2(line: &String) -> i32 {
    let vec = line.split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    for i in 0..vec.len() {
        let mut unsafe_counter = 0;
        if i == 0 { continue; }

        let diff = (vec[i] - vec[i-1]).abs();

        if  diff > 3 || diff < 1  {

            if( i <= 1 ){
                continue;
            }
            // unsafe case
            let diff2 = (vec[i] - vec[i-2]).abs();
            return if diff2 > 3 || diff2 < 1 {
                0
            } else {
                1
            }
        }
    }
    if vec.is_sorted() || vec.is_sorted_by(|x, x1| { x >= x1 }) {
        return 1;
    }
    return 0;

}