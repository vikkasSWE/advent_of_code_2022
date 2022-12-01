use std::cmp::Ordering;
use std::fs;

fn part_1_2() {
    let input = fs::read_to_string("days/day_1/input.txt").unwrap();
    let mut calorie_vec = Vec::new();
    let mut sum = 0;
    for line in input.lines() {
        if line.is_empty() {
            calorie_vec.push(sum);
            sum = 0;
            continue;
        }
        sum = sum + line.parse::<i32>().unwrap();
    }

    let (index, max) = calorie_vec
        .iter()
        .enumerate()
        .max_by(|(_, value_a), (_, value_b)| {
            value_a.partial_cmp(value_b).unwrap_or(Ordering::Equal)
        })
        .unwrap();
    println!("index: {}, Max: {}", index, max);
    calorie_vec.remove(index);

    let (index, max) = calorie_vec
        .iter()
        .enumerate()
        .max_by(|(_, value_a), (_, value_b)| {
            value_a.partial_cmp(value_b).unwrap_or(Ordering::Equal)
        })
        .unwrap();
    println!("index: {}, Max: {}", index, max);
    calorie_vec.remove(index);

    let (index, max) = calorie_vec
        .iter()
        .enumerate()
        .max_by(|(_, value_a), (_, value_b)| {
            value_a.partial_cmp(value_b).unwrap_or(Ordering::Equal)
        })
        .unwrap();
    println!("index: {}, Max: {}", index, max);
    calorie_vec.remove(index);
}

fn main() {
    part_1_2();
}
