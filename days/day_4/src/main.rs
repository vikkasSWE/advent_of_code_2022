use std::{collections::HashMap, fs};
fn part_2(_input: &String) {}

fn main() {
    let input = fs::read_to_string("days/day_4/input.txt").unwrap();
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &String) {
    let mut sum = 0;
    for line in input.lines() {
        let mut split = line.split(",");
        let mut sec_1 = split.next().unwrap();
        let mut sec_2 = split.next().unwrap();

        let mut sec_1 = sec_1.split("-");
        let sec_1_start = sec_1.next().unwrap().parse::<i32>().unwrap();
        let sec_1_end = sec_1.next().unwrap().parse::<i32>().unwrap();

        let mut sec_2 = sec_2.split("-");
        let sec_2_start = sec_2.next().unwrap().parse::<i32>().unwrap();
        let sec_2_end = sec_2.next().unwrap().parse::<i32>().unwrap();

        if sec_1_start >= sec_2_start && sec_1_end <= sec_2_end {
            sum += 1;
            continue;
        }

        if sec_2_start >= sec_1_start && sec_2_end <= sec_1_end {
            sum += 1;
            continue;
        }
    }

    println!("{sum}")
}
