use std::fs;

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("Failed to read input.txt file");

    let mut position: i32 = 50;
    let mut zero_count = 0;

    for line in file_contents.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let direction = line.chars().next().unwrap();
        let distance: i32 = line[1..].parse().expect("Failed to parse distance");

        match direction {
            'L' => {
                position = (position - distance).rem_euclid(100);
            }
            'R' => {
                position = (position + distance) % 100;
            }
            _ => panic!("Invalid direction: {}", direction),
        }

        if position == 0 {
            zero_count += 1;
        }
    }

    println!("The password is: {}", zero_count)
}