use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;

fn main() {
    let input_path = "input.txt";
    let file = File::open(input_path).expect("no such file");
    let mut buf = BufReader::new(file);
    let mut line = String::new();
    buf.read_to_string(&mut line).unwrap();

    const NUM_CHARS: usize = 14;

    let mut current_chars = Vec::new();
    for (idx, char) in line.chars().enumerate() {
        if current_chars.len() >= NUM_CHARS {
            current_chars.remove(0);
        }
        current_chars.push(char);
        
        let unique_chars: HashSet<&char> = HashSet::from_iter(current_chars.iter());
        if unique_chars.len() == NUM_CHARS {
            println!("{:?}", idx + 1);
            break;
        }
    }
}
