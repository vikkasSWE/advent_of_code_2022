use std::fs;

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
            if b == "X" {
                result = Win::Equal;
            } else if b == "Y" {
                result = Win::B;
            } else if b == "Z" {
                result = Win::A;
            }
        } else if a == "B" {
            if b == "X" {
                result = Win::A;
            } else if b == "Y" {
                result = Win::Equal;
            } else if b == "Z" {
                result = Win::B;
            }
        } else if a == "C" {
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

fn part_2(input: &String) {
    let mut points = 0;
    for line in input.lines() {
        let mut round = line.split(' ');
        let a = round.next().unwrap();
        let b = round.next().unwrap();

        let mut desired_outcome = Win::Equal;
        if b == "X" {
            desired_outcome = Win::A;
        } else if b == "Y" {
            desired_outcome = Win::Equal;
        } else if b == "Z" {
            desired_outcome = Win::B;
        }

        let mut choice = "";

        if a == "A" {
            match desired_outcome {
                Win::Equal => choice = "X",
                Win::A => choice = "Z",
                Win::B => choice = "Y",
            }
        } else if a == "B" {
            match desired_outcome {
                Win::Equal => choice = "Y",
                Win::A => choice = "X",
                Win::B => choice = "Z",
            }
        } else if a == "C" {
            match desired_outcome {
                Win::Equal => choice = "Z",
                Win::A => choice = "Y",
                Win::B => choice = "X",
            }
        }

        let mut choice_point = 0;
        if choice == "X" {
            choice_point = 1;
        } else if choice == "Y" {
            choice_point = 2;
        } else if choice == "Z" {
            choice_point = 3;
        }

        match desired_outcome {
            Win::Equal => points = points + 3 + choice_point,
            Win::A => points = points + choice_point,
            Win::B => points = points + 6 + choice_point,
        }
    }

    println!("{points}");
}
