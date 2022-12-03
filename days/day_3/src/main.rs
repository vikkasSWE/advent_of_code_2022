use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("days/day_3/input.txt").unwrap();
    // part_1(&input);
    part_2(&input);
}

fn part_1(input: &String) {
    let priority_map = HashMap::from([
        ("a", 1),
        ("b", 2),
        ("c", 3),
        ("d", 4),
        ("e", 5),
        ("f", 6),
        ("g", 7),
        ("h", 8),
        ("i", 9),
        ("j", 10),
        ("k", 11),
        ("l", 12),
        ("m", 13),
        ("n", 14),
        ("o", 15),
        ("p", 16),
        ("q", 17),
        ("r", 18),
        ("s", 19),
        ("t", 20),
        ("u", 21),
        ("v", 22),
        ("w", 23),
        ("x", 24),
        ("y", 25),
        ("z", 26),
        ("A", 27),
        ("B", 28),
        ("C", 29),
        ("D", 30),
        ("E", 31),
        ("F", 32),
        ("G", 33),
        ("H", 34),
        ("I", 35),
        ("J", 36),
        ("K", 37),
        ("L", 38),
        ("M", 39),
        ("N", 40),
        ("O", 41),
        ("P", 42),
        ("Q", 43),
        ("R", 44),
        ("S", 45),
        ("T", 46),
        ("U", 47),
        ("V", 48),
        ("W", 49),
        ("X", 50),
        ("Y", 51),
        ("Z", 52),
    ]);

    let mut matches = Vec::new();
    'outer: for rucksack in input.lines() {
        let len = rucksack.len() / 2;

        let comp_a = rucksack[0..len].to_string();
        let comp_b = rucksack[len..].to_string();

        for letter_a in comp_a.chars() {
            for letter_b in comp_b.chars() {
                if letter_a == letter_b {
                    println!("{letter_a}");
                    matches.push(letter_a.to_string());
                    continue 'outer;
                }
            }
        }

        break;
    }

    let mut sum = 0;
    for m in matches {
        sum = sum + priority_map.get(m.as_str()).unwrap();
    }

    println!("{sum}");
}

fn part_2(_input: &String) {
    let priority_map = HashMap::from([
        ("a", 1),
        ("b", 2),
        ("c", 3),
        ("d", 4),
        ("e", 5),
        ("f", 6),
        ("g", 7),
        ("h", 8),
        ("i", 9),
        ("j", 10),
        ("k", 11),
        ("l", 12),
        ("m", 13),
        ("n", 14),
        ("o", 15),
        ("p", 16),
        ("q", 17),
        ("r", 18),
        ("s", 19),
        ("t", 20),
        ("u", 21),
        ("v", 22),
        ("w", 23),
        ("x", 24),
        ("y", 25),
        ("z", 26),
        ("A", 27),
        ("B", 28),
        ("C", 29),
        ("D", 30),
        ("E", 31),
        ("F", 32),
        ("G", 33),
        ("H", 34),
        ("I", 35),
        ("J", 36),
        ("K", 37),
        ("L", 38),
        ("M", 39),
        ("N", 40),
        ("O", 41),
        ("P", 42),
        ("Q", 43),
        ("R", 44),
        ("S", 45),
        ("T", 46),
        ("U", 47),
        ("V", 48),
        ("W", 49),
        ("X", 50),
        ("Y", 51),
        ("Z", 52),
    ]);

    let mut lines = _input.lines();
    let num_three = lines.clone().count() / 3;
    let mut mathes = Vec::new();

    'outer: for _ in 0..num_three {
        let line_1 = lines.next().unwrap();
        let line_2 = lines.next().unwrap();
        let line_3 = lines.next().unwrap();

        for letter_1 in line_1.chars() {
            for letter_2 in line_2.chars() {
                if letter_1 == letter_2 {
                    for letter_3 in line_3.chars() {
                        if letter_1 == letter_3 {
                            println!("{letter_1}");
                            mathes.push(letter_1.to_string());
                            continue 'outer;
                        }
                    }
                }
            }
        }
    }

    let mut sum = 0;
    for m in mathes {
        sum = sum + priority_map.get(m.as_str()).unwrap();
    }

    println!("{sum}");
}
