use std::fs;

fn is_number_invalid_id(n: u64) -> bool {
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

fn solve(input: &str) -> (u64, Vec<u64>) {
    let mut total = 0u64;
    let mut invalid_ids = Vec::new();

    for part in input.trim().split(',') {
        let parts: Vec<&str> = part.trim().split('-').collect();
        if parts.len() != 2 {
            continue;
        }

        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();

        println!("start {}", start);
        println!("end {}", end);

        for n in start..=end {
            if is_number_invalid_id(n) {
                invalid_ids.push(n);
                total += n;
            }
        }
    }

    (total, invalid_ids)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let filename = &args[1];

    let puzzle_input = fs::read_to_string(filename)
        .unwrap_or_else(|e| {
            eprintln!("Error reading file '{}': {}", filename, e);
            std::process::exit(1);
        });

    let (total, invalid_ids) = solve(&puzzle_input);

    println!("invalid ids {:?}", invalid_ids);
    println!("total {}", total);
}
