use std::{
    collections::{HashMap, VecDeque},
    fs,
};

fn main() {
    let crates = fs::read_to_string("days/day_5/crates.txt").unwrap();
    let input = fs::read_to_string("days/day_5/input.txt").unwrap();
    part_1(&input, &crates);
    part_2(&input, &crates);
}

fn part_2(input: &String, crates: &String) {}

fn part_1(input: &String, crates: &String) {
    let mut stacks = HashMap::new();
    // stacks.insert(1, VecDeque::from(["N", "Z"]));
    // stacks.insert(2, VecDeque::from(["D", "C", "M"]));
    // stacks.insert(3, VecDeque::from(["P"]));

    stacks.insert(
        1,
        VecDeque::from(["N", "T", "B", "S", " Q", "H", " G", "R"]),
    );
    stacks.insert(2, VecDeque::from(["J", "Z", "P", "D", "F", "S", "H"]));
    stacks.insert(3, VecDeque::from(["V", "H", "Z"]));
    stacks.insert(4, VecDeque::from(["H", "G", "F", "J", "Z", "M"]));
    stacks.insert(5, VecDeque::from(["R", "S", "M", "L", "D", "C", "Z", "T"]));
    stacks.insert(6, VecDeque::from(["J", "Z", "H", "V", "W", "T", "M"]));
    stacks.insert(7, VecDeque::from(["Z", "L", "P", "F", "T"]));
    stacks.insert(8, VecDeque::from(["S", "W", "V", "Q"]));
    stacks.insert(9, VecDeque::from(["C", "N", "D", "T", "M", "L", "H", "W"]));

    for line in input.lines() {
        let mut split = line.split_whitespace();
        split.next().unwrap();
        let number_moved_crates = split.next().unwrap().parse::<i32>().unwrap();
        split.next().unwrap();
        let from_stack = split.next().unwrap().parse::<i32>().unwrap();
        split.next().unwrap();
        let to_stack = split.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..number_moved_crates {
            let from = stacks.get_mut(&from_stack).unwrap();

            if let Some(value) = from.pop_front() {
                let to = stacks.get_mut(&to_stack).unwrap();

                to.push_front(value.clone());
            }
        }
    }
    println!("{:?}", stacks.get(&1));
    println!("{:?}", stacks.get(&2));
    println!("{:?}", stacks.get(&3));
    println!("{:?}", stacks.get(&4));
    println!("{:?}", stacks.get(&5));
    println!("{:?}", stacks.get(&6));
    println!("{:?}", stacks.get(&7));
    println!("{:?}", stacks.get(&8));
    println!("{:?}", stacks.get(&9));
}
