use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main_a() {

    let input_path = "input.txt";
    let file = File::open(input_path).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.expect("couldnt parse line")).collect();

    let mut score = 0;

    for pair in lines {
        let mut split = pair.split(" ");
        let choices = (split.next().unwrap(), split.next().unwrap());

        score += {
            if choices.0 == "A" {
                match choices.1 {
                    "X" => 1 + 3,
                    "Y" => 2 + 6,
                    "Z" => 3 + 0,
                    _ => panic!("panikk"),
                }
            } else if choices.0 == "B" {
                match choices.1 {
                    "X" => 1 + 0,
                    "Y" => 2 + 3,
                    "Z" => 3 + 6,
                    _ => panic!("panikk"),
                }
            } else {
                match choices.1 {
                    "X" => 1 + 6,
                    "Y" => 2 + 0,
                    "Z" => 3 + 3,
                    _ => panic!("panikk"),
                }
            }
        };
    }

    println!("{:?}", score);
}

fn main() {

    let input_path = "input.txt";
    let file = File::open(input_path).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.expect("couldnt parse line")).collect();

    let mut score = 0;

    for pair in lines {
        let mut split = pair.split(" ");
        let choices = (split.next().unwrap(), split.next().unwrap());

        score += {
            if choices.0 == "A" {
                match choices.1 {
                    "X" => 0 + 3,
                    "Y" => 3 + 1,
                    "Z" => 6 + 2,
                    _ => panic!("panikk"),
                }
            } else if choices.0 == "B" {
                match choices.1 {
                    "X" => 0 + 1,
                    "Y" => 3 + 2,
                    "Z" => 6 + 3,
                    _ => panic!("panikk"),
                }
            } else {
                match choices.1 {
                    "X" => 0 + 2,
                    "Y" => 3 + 3,
                    "Z" => 6 + 1,
                    _ => panic!("panikk"),
                }
            }
        };
    }

    println!("{:?}", score);
}