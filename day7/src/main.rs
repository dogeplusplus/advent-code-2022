use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Clone)]
struct TreeNode {
    pub identifier: String,
    pub parent: Option<usize>,
    pub value: Option<u32>,
    pub children: Vec<usize>,
}

fn size(nodes: &Vec<TreeNode>, idx: usize) -> u32 {
    if let Some(v) = nodes[idx].value {
        v
    } else {
        nodes[idx].children.iter().map(|x| size(nodes, *x)).sum()
    }
}

fn main() {
    let input_path = "input.txt";
    let file = File::open(input_path).expect("no such file");
    let buf = BufReader::new(file);
    let lines: Vec<String> = buf.lines().map(|l| l.expect("couldnt parse line")).collect();

    let root = TreeNode {
        identifier: "/".to_string(),
        parent: None,
        value: None,
        children: Vec::new(),
    };


    let mut nodes = Vec::new();
    nodes.push(root);

    let mut current_node = 0;
    let mut current_len = 1;

    for line in lines {
        // Check if moving up a directory, into a root, or a subfolder
        if line.starts_with("$ cd") {
            let directory = line.split(" ").last().unwrap();
            if directory == ".." {
                if let Some(parent) = nodes[current_node].parent {
                    current_node = parent;
                }
            } else if directory == "/" {
                current_node = 0;
            } else {
                for child in nodes[current_node].clone().children {
                    if nodes[child].identifier == directory {
                        current_node = child;
                        break;
                    }
                }
            }
        // Every other line should be an ls or a folder/file so can skip here
        } else if line.starts_with("$ ls") {
            continue;
        } else {
            // Folder or file

            // File case
            if line.starts_with("dir ") {
                // Check if directory exists already
                let directory = line.split(" ").last().unwrap();
                let mut found = false;
                for child in nodes.clone()[current_node].children.iter() {
                    if nodes[*child].identifier == directory {
                        found = true;
                    }
                }
                // Add child node directory
                if !found {
                    let new_node = TreeNode {
                            identifier: directory.to_string(),
                            parent: Some(current_node),
                            value: None,
                            children: Vec::new(),
                    };
                    nodes.push(new_node);
                    nodes[current_node].children.push(current_len);
                    current_len += 1;
                }
            } else {
                // Otherwise its a file
                let value_name: Vec<&str> = line.split(" ").collect();
                let value = value_name[0].parse::<u32>().unwrap();
                let name = value_name[1];
                // Check if file already exists in the children
                let mut found = false;
                for child in nodes[current_node].children.iter() {
                    if nodes[*child].identifier == name {
                        found = true;
                    }
                }
                // Add child node with value
                if !found {
                    let new_node = TreeNode {
                            identifier: name.to_string(),
                            parent: Some(current_node),
                            value: Some(value),
                            children: Vec::new(),
                    };
                    nodes.push(new_node);
                    nodes[current_node].children.push(current_len);
                    current_len += 1;
                }
            }
        }
    }

    let mut folder_sizes = Vec::new();
    let mut total_size_under_10k = 0;
    let mut total_size = 0;

    for idx in 0..nodes.len() {
        if let Some(file_size) = nodes[idx].value {
            total_size += file_size;
        } else {
            let folder_size = size(&nodes, idx);
            if folder_size <= 100000 {
                total_size_under_10k += folder_size;
            }
            folder_sizes.push(folder_size);
        } 
    }

    println!("{:?}", total_size_under_10k);


    let target_space = 40000000;
    let free_up = total_size - target_space;
    println!("Total Size: {:?}", total_size);
    println!("Smallest folder to free up: {:?}", folder_sizes.iter().filter(|x| **x >= free_up).min());
}