use std::fs;

fn count_zero_crossings(position: i32, direction: char, steps: i32) -> i32 {
    if direction == 'R' {
        (position + steps) / 100
    } else {
        if position == 0 {
            steps / 100
        } else if steps >= position {
            1 + (steps - position) / 100
        } else {
            0
        }
    }
}

fn simulate_dial_rotations(input: &str) -> (i32, i32) {
    let mut position = 50;
    let mut part1_count = 0;
    let mut part2_count = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        println!("line {}", line);

        let direction = line.chars().next().unwrap();
        let steps: i32 = line[1..].parse().unwrap();

        let crossings = count_zero_crossings(position, direction, steps);
        part2_count += crossings;

        if direction == 'R' {
            position = (position + steps) % 100;
        } else {
            position = ((position - steps) % 100 + 100) % 100;
        }

        if position == 0 {
            part1_count += 1;
        }
    }

    (part1_count, part2_count)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let filename = &args[1];
    let puzzle_input = fs::read_to_string(filename).unwrap_or_else(|e| {
        eprintln!("Error reading file '{}': {}", filename, e);
        std::process::exit(1);
    });

    let (part1, part2) = simulate_dial_rotations(&puzzle_input);
    println!("Times landing on 0 {}", part1);
    println!("all zero crossings {}", part2);
}
