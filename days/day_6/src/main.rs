use std::fs;

fn main() {
    let input = fs::read_to_string("days/day_6/input.txt").unwrap();
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &String) {
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
            println!("{}", index + 1);
            break;
        }
    }
}

use array_tool::vec::Uniq;

fn part_2(input: &String) {
    let mut window = Vec::new();
    for _ in 0..14 {
        window.push(String::new());
    }

    for (index, character) in input.chars().enumerate() {
        for index in (0..14).rev() {
            if index != 0 {
                window[index] = window[index - 1].clone();
            } else {
                window[index] = character.to_string();
            }
        }

        let test = window.to_vec();

        if test.is_unique() {
            println!("{}", index + 1);
            break;
        }
    }
}
