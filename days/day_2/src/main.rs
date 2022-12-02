use std::fs;

fn part_2(_input: &String) {}

fn main() {
    let input = fs::read_to_string("days/day_2/input.txt").unwrap();
    part_1(&input);
    part_2(&input);
}

enum Win {
    Equal,
    A,
    B,
}

fn part_1(input: &String) {
    let mut points = 0;
    for line in input.lines() {
        let mut round = line.split(' ');
        let a = round.next().unwrap();
        let b = round.next().unwrap();

        let mut result = Win::Equal;

        if a == "A" {
            // rock
            if b == "X" {
                // rock
                result = Win::Equal;
            } else if b == "Y" {
                // paper
                result = Win::B;
            } else if b == "Z" {
                // scissor
                result = Win::A;
            }
        } else if a == "B" {
            // paper
            if b == "X" {
                result = Win::A;
            } else if b == "Y" {
                result = Win::Equal;
            } else if b == "Z" {
                result = Win::B;
            }
        } else if a == "C" {
            // scissor
            if b == "X" {
                result = Win::B;
            } else if b == "Y" {
                result = Win::A;
            } else if b == "Z" {
                result = Win::Equal;
            }
        }

        let mut choice_point = 0;

        if b == "X" {
            choice_point = 1;
        } else if b == "Y" {
            choice_point = 2;
        } else if b == "Z" {
            choice_point = 3;
        }

        match result {
            Win::Equal => points = points + 3 + choice_point,
            Win::A => points = points + choice_point,
            Win::B => points = points + 6 + choice_point,
        }
    }

    println!("{points}");
}
