use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main_a() {
    let input_path = "input.txt";
    let file = File::open(input_path).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("couldnt parse line"))
        .collect();

    let mut current_value = 1;
    let mut register = Vec::new();
    register.push(current_value);

    for line in lines {
        let command = line.split(" ").next().unwrap();

        if command == "noop" {
            register.push(current_value);
        } else {
            let value = line.split(" ").last().unwrap().parse::<i32>().unwrap();
            register.push(current_value);
            current_value += value;
            register.push(current_value);
        }
    }

    // println!("{:?}", register);

    let indices = vec![20, 60, 100, 140, 180, 220];

    let mut total = 0;

    for idx in indices {
        total += (idx as i32) * register[idx-1];
    }

    println!("{:?}", total);
}

fn main() {
    let input_path = "input.txt";
    let file = File::open(input_path).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("couldnt parse line"))
        .collect();

    let mut current_value = 1;
    let mut register = Vec::new();
    register.push(current_value);

    for line in lines {
        let command = line.split(" ").next().unwrap();

        if command == "noop" {
            register.push(current_value);
        } else {
            let value = line.split(" ").last().unwrap().parse::<i32>().unwrap();
            register.push(current_value);
            current_value += value;
            register.push(current_value);
        }
    }

    let mut result = Vec::new();

    for idx in 0..register.len() {
        let mut current = register[idx];
        current = current % 40;
        let idx = idx % 40;
    
        if idx as i32 - 1 <= current && current <= idx as i32 + 1 {
            result.push("#");
        } else {
            result.push(".");
        }
    }

    let chunk_size = 40;
    let lines: Vec<String> = result.chunks(chunk_size).map(|x| x.join("")).collect();
    for line in lines {
        println!("{:?}", line);
    }

}