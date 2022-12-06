use std::fs;

fn main() {
    let input = fs::read_to_string("days/day_6/input.txt").unwrap();
    part_1(&input);
    part_2(&input);
}

fn part_2(_input: &String) {}

fn part_1(input: &String) {
    // for line in input.lines() {
    let mut v1 = "".to_string();
    let mut v2 = "".to_string();
    let mut v3 = "".to_string();
    let mut v4 = "".to_string();

    for (index, character) in input.chars().enumerate() {
        v4 = v3.clone();
        v3 = v2.clone();
        v2 = v1.clone();
        v1 = character.to_string();

        if (v1 != v2)
            && (v1 != v3)
            && (v1 != v4)
            && (v2 != v3)
            && (v2 != v4)
            && (v3 != v4)
            && (v4 != "")
        {
            println!("v1: {v1}");
            println!("v2: {v2}");
            println!("v3: {v3}");
            println!("v4: {v4}");
            println!("{}", index + 1);
            break;
        }
    }
    // }
}
