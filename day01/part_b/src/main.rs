use aoc_utils::read_input;

fn main() {
    let input = read_input("../inputs/day01/input_b.txt");
    let answer = solve(&input);
    println!("{answer}");
}

fn solve(input: &str) -> String {
    let input = input.trim_end();

    let mut position: i32 = 50;
    let mut zero_count = 0;

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with("#") {
            continue;
        }

        let direction = line.chars().next().expect("Empty line");
        let distance_str = line[1..].trim();

        let distance: i32 = match distance_str.parse() {
            Ok(d) => d,
            Err(e) => {
                eprintln!(
                    "Failed to parse '{}' from line '{}': {}",
                    distance_str, line, e
                );
                panic!("Invalid distance in line: '{}'", line);
            }
        };

        let hits = match direction {
            'R' => (position + distance) / 100 - position / 100,
            'L' => {
                if position == 0 {
                    distance / 100
                } else {
                    distance / 100 + if distance % 100 >= position { 1 } else { 0 }
                }
            }
            _ => panic!("Invalid direction '{}' in line: '{}'", direction, line),
        };

        zero_count += hits;

        position = match direction {
            'L' => (position - distance).rem_euclid(100),
            'R' => (position + distance).rem_euclid(100),
            _ => unreachable!(),
        };
    }

    zero_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(solve(input), "6");
    }

    #[test]
    fn test_multiple_wraps() {
        // R1000 from position 50 should cross 0 ten times
        let input = "R1000";
        assert_eq!(solve(input), "10");
    }

    #[test]
    fn test_single_crossing() {
        // From 50, R50 should hit 0 once (landing at 0)
        let input = "R50";
        assert_eq!(solve(input), "1");

        // From 50, L68 should hit 0 once during rotation
        let input2 = "L68";
        assert_eq!(solve(input2), "1");
    }

    #[test]
    fn test_no_crossings() {
        // From 50, R10 doesn't hit 0
        let input = "R10";
        assert_eq!(solve(input), "0");
    }
}
