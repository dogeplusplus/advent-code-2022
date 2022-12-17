use std::fs::File;
use std::io::{prelude::*, BufReader};
use ndarray::{Array2,Array1};
use std::cmp::{max,min};

fn main_a() {
    let input_path = "input.txt";
    let file = File::open(input_path).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.expect("couldnt parse line")).collect();

    let nrows = lines.len();
    let ncols = lines.first().map_or(0, |row| row.len());

    let mut trees = Vec::new();
    for line in lines {
        let row: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        trees.extend(row);
    }

    let trees: Array2<u32> = Array2::from_shape_vec((nrows, ncols), trees).unwrap();
    
    let mut left = Array2::zeros((nrows, ncols));
    let mut current_left = Array1::zeros(nrows);
    for col in 0..ncols {
        for row in 0..nrows {
            left[[row, col]] = current_left[row];
            current_left[row] = max(trees[[row, col]], current_left[row]);
        }
    }

    let mut right = Array2::zeros((nrows, ncols));
    let mut current_right = Array1::zeros(nrows);
    for col in (0..ncols).rev() {
        for row in 0..nrows {
            right[[row, col]] = current_right[row];
            current_right[row] = max(trees[[row,col]], current_right[row]);
        }
    }

    let mut top = Array2::zeros((nrows, ncols));
    let mut current_top = Array1::zeros(ncols);
    for row in 0..nrows  {
        for col in 0..ncols {
            top[[row,col]] = current_top[col];
            current_top[col] = max(trees[[row,col]], current_top[col]);
        }
    }
    
    let mut bottom = Array2::zeros((nrows, ncols));
    let mut current_bottom = Array1::zeros(ncols);
    for row in (0..nrows).rev()  {
        for col in 0..ncols {
            bottom[[row,col]] = current_bottom[col];
            current_bottom[col] = max(trees[[row,col]], current_bottom[col]);
        }
    }

    let mut visible_trees = 0;
    for row in 0..nrows {
        for col in 0..ncols {
            let min_adjacent = min(min(left[[row, col]], top[[row, col]]), min(right[[row, col]], bottom[[row, col]]));
            if (trees[[row, col]] > min_adjacent) || row == 0 || col == 0 || row == (nrows - 1) || col == (ncols - 1) {
                visible_trees += 1;
            }
        }
    }
    println!("{:?}", visible_trees);
}

fn scenic_score(trees: &Array2<u32>, row: usize, col: usize) -> u32 {
    let nrows = trees.dim().0;
    let ncols = trees.dim().1;

    let mut left = 0;
    if col != 0 {
        left = 1;
        for i in (1..col).rev() {
            if trees[[row, col]] > trees[[row, i]] {
                left += 1;
            } else {
                break;
            }        
        }
    } 

    let mut right = 0;
    if col != ncols - 1 {
        right = 1;
        for i in col+1..ncols-1 {
            if trees[[row, col]] > trees[[row, i]] {
                right += 1;
            } else {
                break;
            }        
        }
    }

    let mut top = 0;
    if row != 0 {
        top = 1;
        for i in (1..row).rev() {
            if trees[[row, col]] > trees[[i, col]] {
                top += 1;
            } else {
                break;
            }        
        }
    }

    let mut bottom = 0;
    if row != nrows - 1 {
        bottom = 1;
        for i in row+1..nrows-1 {
            if trees[[row, col]] > trees[[i, col]] {
                bottom += 1;
            } else {
                break;
            }        
        }
    }

    left * right * bottom * top
}

fn main() {
    let input_path = "input.txt";
    let file = File::open(input_path).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.expect("couldnt parse line")).collect();

    let nrows = lines.len();
    let ncols = lines.first().map_or(0, |row| row.len());

    let mut trees = Vec::new();
    for line in lines {
        let row: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        trees.extend(row);
    }

    let trees: Array2<u32> = Array2::from_shape_vec((nrows, ncols), trees).unwrap();
    
    let mut max_viewing = 0;
    for row in 0..nrows {
        for col in 0..ncols {
            let viewing = scenic_score(&trees, row, col);
            if viewing > max_viewing {
                max_viewing = viewing;
            }
        }
    }

    println!("{:?}", max_viewing);
}