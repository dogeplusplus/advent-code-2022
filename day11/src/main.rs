fn value_of(from: usize, old: i64) -> i64 {
    let factor = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
    
    let new = match from {
        0 => old * 13,
        1 => old + 7,
        2 => old + 2,
        3 => old * 2,
        4 => old * old,
        5 => old + 6,
        6 => old + 1,
        7 => old + 8,
        _ => panic!("panik"),
    };

    new % factor
}
fn throw_to(from: usize, to: [usize; 2], old: i64) -> usize  {
    let new = value_of(from, old);
    let test = match from {
        0 => new % 2 == 0, 
        1 => new % 13 == 0,
        2 => new % 5 == 0,
        3 => new % 3 == 0,
        4 => new % 11 == 0,
        5 => new % 17 == 0,
        6 => new % 7 == 0,
        7 => new % 19 == 0,
        _ => panic!("panik"),
    };

    match test {
        true => to[0],
        false => to[1],
    }
}
fn main() {
    let mut inventory = vec![
        vec![91, 54, 70, 61, 64, 64, 60, 85],
        vec![82],
        vec![84, 93, 70],
        vec![78, 56, 85, 93],
        vec![64, 57, 81, 95, 52, 71, 58],
        vec![58, 71, 96, 58, 68, 90],
        vec![56, 99, 89, 97, 81],
        vec![68, 72],
    ];

    let to = vec![
        [5, 2],
        [4, 3],
        [5, 1],
        [6, 7],
        [7, 3],
        [4, 1],
        [0, 2],
        [6, 0],
    ];

    let rounds = 10000;
    let mut inspected = Vec::from([0; 8]);

    for _ in 0..rounds {
        for m in 0..8 {
            inspected[m] += inventory[m].len();

            for item in inventory[m].clone() {
                let to = throw_to(m, to[m], item);
                inventory[to].push(value_of(m, item));
            }
            inventory[m] = vec![];
        }
    }

    inspected.sort();
    inspected.reverse();
    println!("{:?}", inspected[0] * inspected[1]);
}