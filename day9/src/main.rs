use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Tuple (i32, i32);

fn tail_moves(head: Tuple, tail: Tuple) -> bool {
    if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
        true
    } else {
        false
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Tuple(self.0 + other.0, self.1 + other.1)
    }
}

impl Mul<i32> for Tuple {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Tuple(self.0 * other, self.1 * other)
    }
}

#[allow(dead_code)]
fn main_a() {
    let input_path = "input.txt";
    let file = File::open(input_path).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.expect("couldnt parse line")).collect();

    let mut head = Tuple(0, 0);
    let mut tail = Tuple(0, 0);

    let mut visited: HashSet<Tuple> = HashSet::new();
    visited.insert(tail);

    for line in lines {
        let direction = line.split(" ").next().unwrap();
        let length = line.split(" ").last().unwrap().parse::<i32>().unwrap();
        let vector = match direction {
            "D" => Tuple(1, 0),
            "U" => Tuple(-1, 0),
            "L" => Tuple(0, -1),
            "R" => Tuple(0, 1),
            _ => panic!("panikk"),
        };

        for _ in 0..length {
            let old_head = head;
            head = head + vector;
            if tail_moves(head, tail) {
                tail = old_head;
                visited.insert(tail);
            }
        }
    }

    println!("Tail visited: {:?}", visited.len());
}

fn main() {
    let input_path = "input_test_1.txt";
    let file = File::open(input_path).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.expect("couldnt parse line")).collect();

    let mut segments = vec![Tuple(0, 0); 10];

    let mut visited: HashSet<Tuple> = HashSet::new();
    visited.insert(Tuple(0, 0));

    for line in lines {
        let direction = line.split(" ").next().unwrap();
        let length = line.split(" ").last().unwrap().parse::<i32>().unwrap();
        let vector = match direction {
            "D" => Tuple(1, 0),
            "U" => Tuple(-1, 0),
            "L" => Tuple(0, -1),
            "R" => Tuple(0, 1),
            _ => panic!("panikk"),
        };

        for _ in 0..length {
            segments[0] = segments[0] + vector;
            let direction = Tuple(segments[0].0 - segments[1].0, segments[0].1 - segments[1].1);
            for i in 1..segments.len() {
                if tail_moves(segments[i-1], segments[i]) {
                    segments[i] = segments[i] + direction;

                } 
            }
            visited.insert(segments[9]);
        }
    }

    println!("Tail visited: {:?}", visited.len());
}