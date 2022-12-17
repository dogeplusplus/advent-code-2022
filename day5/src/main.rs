use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main_a() {
    let input_path = "input.txt";
    let file = File::open(input_path).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.expect("couldnt parse line")).collect();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..9 {
        stacks.push(Vec::new());
    }
    for line in &lines[0..8] {
        for (stack_num, idx) in (1..=33).step_by(4).enumerate() {
            if line.chars().nth(idx).unwrap() != ' ' {
                stacks[stack_num].insert(0, line.chars().nth(idx).unwrap());
            }
        }
    }

    for line in &lines[10..] {
        let words: Vec<&str> = line.split(" ").collect();
        let num = words[1].parse::<u32>().unwrap();
        let from = words[3].parse::<usize>().unwrap();
        let to = words[5].parse::<usize>().unwrap();

        for _ in 0..num {
            if let Some(item) = stacks[from-1].pop() {
                stacks[to-1].push(item);
            }
        }
    }

    for mut stack in stacks {
        print!("{:?}", stack.pop().unwrap());
    }
}

fn main() {
    let input_path = "input.txt";
    let file = File::open(input_path).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.expect("couldnt parse line")).collect();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..9 {
        stacks.push(Vec::new());
    }
    for line in &lines[0..8] {
        for (stack_num, idx) in (1..=33).step_by(4).enumerate() {
            if line.chars().nth(idx).unwrap() != ' ' {
                stacks[stack_num].insert(0, line.chars().nth(idx).unwrap());
            }
        }                
    }

    for line in &lines[10..] {
        let words: Vec<&str> = line.split(" ").collect();
        let num = words[1].parse::<u32>().unwrap();
        let from = words[3].parse::<usize>().unwrap();
        let to = words[5].parse::<usize>().unwrap();

        let mut temp: Vec<char> = Vec::new();
        for _ in 0..num {
            if let Some(item) = stacks[from-1].pop() {
                temp.insert(0, item);
            }
        }
        for item in temp {
            stacks[to-1].push(item);
        }
    }

    for mut stack in stacks {
        print!("{:?}", stack.pop().unwrap());        
    }
}