use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main_a() {
    let input_path = "input.txt";
    let file = File::open(input_path).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.expect("couldnt parse line")).collect();

    let mut priority_map = HashMap::new();

    for (i, c) in ('a'..='z').into_iter().enumerate() {
        priority_map.insert(c, i+1);
    }
    for (i, c) in ('A'..='Z').into_iter().enumerate() {
        priority_map.insert(c, i+27);
    }

    let mut total_priority = 0;
    for line in lines {
        let midpoint = line.len() / 2;
        let mut left_section = HashSet::new();
        let mut right_section = HashSet::new();
        for (i, c) in line.chars().enumerate() {
            if i < midpoint {
                left_section.insert(c);
            } else {
                right_section.insert(c);
            }
        }

        let intersection = left_section.intersection(&right_section);
        for inter in intersection {
            total_priority += priority_map.get(inter).unwrap();
        }
    }

    println!("{:?}", total_priority);
}

fn main() {
    let input_path = "input.txt";
    let file = File::open(input_path).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.expect("couldnt parse line")).collect();

    let mut priority_map = HashMap::new();

    for (i, c) in ('a'..='z').into_iter().enumerate() {
        priority_map.insert(c, i+1);
    }
    for (i, c) in ('A'..='Z').into_iter().enumerate() {
        priority_map.insert(c, i+27);
    }

    let mut total_priority = 0;
    for lines in lines.chunks(3) {
        let elf1: HashSet<_> = lines[0].chars().collect();
        let elf2: HashSet<_> = lines[1].chars().collect();
        let elf3: HashSet<_> = lines[2].chars().collect();

        let inter = elf1.intersection(&elf2).map(|x| *x).collect();
        let intersection = elf3.intersection(&inter);
        for inter in intersection {
            total_priority += priority_map.get(inter).unwrap();
        }
    }

    println!("{:?}", total_priority);
}
