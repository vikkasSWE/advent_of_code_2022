use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("days/day_7/test.txt").unwrap();
    part_1(&input);
    part_2(&input);
}

fn part_2(_input: &String) {}

#[derive(Debug, Clone)]
struct Directory {
    files_and_sizes: Vec<(String, i32)>,
    directories: Vec<String>,
    total_size: i32,
}

fn part_1(input: &String) {
    let mut current_dir = "/";
    let mut last_dir = "/";
    let mut dir_size_map: HashMap<&str, Directory> = HashMap::new();
    for line in input.lines() {
        if line.contains("cd ") {
            let mut tmp = line.split_whitespace();
            tmp.next();
            tmp.next();

            let test = tmp.next().unwrap();

            current_dir = if test == ".." { last_dir } else { test };
            last_dir = current_dir;

            if let Some(_) = dir_size_map.get_mut(current_dir) {
            } else {
                dir_size_map.insert(
                    current_dir,
                    Directory {
                        files_and_sizes: Vec::new(),
                        total_size: 0,
                        directories: Vec::new(),
                    },
                );
            }
            continue;
        }

        if line.eq("$ ls") {
            continue;
        }

        if line.contains("dir ") {
            // Directory
            let (_, dir) = line.split_once(" ").unwrap();

            if let Some(directory) = dir_size_map.get_mut(current_dir) {
                (*directory).directories.push(dir.to_string())
            }
        } else {
            // File
            let (size, file) = line.split_once(" ").unwrap();
            // println!("{},{}", size, file);
            let size = size.parse::<i32>().unwrap();

            if let Some(directory) = dir_size_map.get_mut(current_dir) {
                (*directory).total_size += size;
                (*directory).files_and_sizes.push((file.to_string(), size));
            }
        }
    }

    let mut sum = 0;
    let dir_size_map_clone = dir_size_map.clone();
    println!("{:?}", dir_size_map);
    for entry in dir_size_map {
        if entry.0 == "/" {
            continue;
        }
        for dir in entry.1.directories {
            sum += dir_size_map_clone.get(&dir.as_str()).unwrap().total_size;
        }
        if entry.1.total_size > 100000 {
            sum += entry.1.total_size;
        }
    }

    println!("{sum}");
}
