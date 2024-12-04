mod day2;
mod day3;

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
