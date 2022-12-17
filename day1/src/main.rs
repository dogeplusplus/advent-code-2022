use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let input_path = "input.txt";
    let file = File::open(input_path).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.expect("couldnt parse line")).collect();

    let mut current = 0;
    let mut calorie_counts = Vec::new();

    for cal in lines {
        if cal == "" {
            calorie_counts.push(current);
            current = 0;
        } else {
            current += cal.parse::<i32>().unwrap();
        }
    }

    if current > 0 {
        calorie_counts.push(current);
    }
    calorie_counts.sort();
    calorie_counts.reverse();
    println!("{:?}", calorie_counts[0] + calorie_counts[1] + calorie_counts[2]);
}
