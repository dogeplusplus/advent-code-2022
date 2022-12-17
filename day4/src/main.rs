use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main_a() {

    let input_path = "input.txt";
    let file = File::open(input_path).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.expect("couldnt parse line")).collect();

    let mut fully_contains = 0;
    for line in lines {
        let ranges: Vec<_> = line.split(",").collect();
        let range_a: Vec<u32> = ranges[0].split("-").map(|x| x.parse::<u32>().unwrap()).collect();
        let range_b: Vec<u32> = ranges[1].split("-").map(|x| x.parse::<u32>().unwrap()).collect();

        if range_a[0] <= range_b[0] && range_b[1] <= range_a[1] {
            fully_contains += 1;
        } else if range_b[0] <= range_a[0] && range_a[1] <= range_b[1] {
            fully_contains += 1;
        }
    }

    println!("{:?}", fully_contains);
}

fn main() {

    let input_path = "input.txt";
    let file = File::open(input_path).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.expect("couldnt parse line")).collect();

    let mut intersects = 0;
    for line in lines {
        let ranges: Vec<_> = line.split(",").collect();
        let range_a: Vec<u32> = ranges[0].split("-").map(|x| x.parse::<u32>().unwrap()).collect();
        let range_b: Vec<u32> = ranges[1].split("-").map(|x| x.parse::<u32>().unwrap()).collect();

        if range_a[1] >= range_b[0] && range_a[1] <= range_b[1] {
            intersects += 1;
        } else if range_a[0] >= range_b[0] && range_a[1] <= range_b[1] {
            intersects += 1;
        } else if range_b[0] >= range_a[0] && range_b[0] <= range_a[1] {
            intersects += 1;
        } else if range_b[1] >= range_a[0] && range_b[1] <= range_a[1] {
            intersects += 1;
        }
    }
    println!("{:?}", intersects);
}