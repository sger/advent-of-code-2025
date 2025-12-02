use std::fs;

fn has_pattern_repeated_twice(n: u64) -> bool {
    let s = n.to_string();
    let length = s.len();

    if length % 2 != 0 {
        return false;
    }

    let mid = length / 2;
    let left = &s[..mid];
    let right = &s[mid..];

    left == right
}

fn has_repeated_pattern(n: u64) -> bool {
    let s = n.to_string();
    let length = s.len();

    for pattern_length in 1..=(length / 2) {
        if length % pattern_length != 0 {
            continue;
        }

        let pattern = &s[0..pattern_length];
        let mut is_repeating = true;

        for i in (pattern_length..length).step_by(pattern_length) {
            if &s[i..i + pattern_length] != pattern {
                is_repeating = false;
                break;
            }
        }

        if is_repeating {
            return true;
        }
    }

    false
}

fn calculate_invalid_id_sums(input: &str) -> (u64, u64) {
    let mut ranges = Vec::new();

    for part in input.trim().split(',') {
        let parts: Vec<&str> = part.trim().split("-").collect();

        if parts.len() != 2 {
            continue;
        }

        let start = parts[0].parse().unwrap();
        let end = parts[1].parse().unwrap();
        ranges.push((start, end));
    }

    let mut part1_total = 0u64;
    let mut part2_total = 0u64;

    for (start, end) in ranges {
        for n in start..=end {
            if has_pattern_repeated_twice(n) {
                part1_total += n
            }

            if has_repeated_pattern(n) {
                part2_total += n
            }
        }
    }

    (part1_total, part2_total)
}
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let filename = &args[1];
    let puzzle_input = fs::read_to_string(filename).unwrap_or_else(|e| {
        eprintln!("Error reading file '{}': {}", filename, e);
        std::process::exit(1);
    });

    let (part1, part2) = calculate_invalid_id_sums(&puzzle_input);
    println!("exactly twice {}", part1);
    println!("at least twice {}", part2);
}
