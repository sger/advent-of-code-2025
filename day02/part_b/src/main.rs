use aoc_utils::read_input;

fn main() {
    let input = read_input("../inputs/day02/input.txt");
    let answer = solve(&input);
    println!("{answer}");
}

fn is_invalid_id(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    if len < 2 {
        return false;
    }

    for pattern_len in 1..=len / 2 {
        if len % pattern_len == 0 {
            let pattern = &s[0..pattern_len];

            let repeated = pattern.repeat(len / pattern_len);

            if repeated == s {
                return true;
            }
        }
    }

    false
}

fn solve(input: &str) -> String {
    let mut total = 0u64;

    for range_str in input.trim().split(',') {
        let parts: Vec<&str> = range_str.trim().split('-').collect();
        let start: u64 = parts[0].parse().expect("Failed to parse start");
        let end: u64 = parts[1].parse().expect("Failed to parse end");

        for id in start..=end {
            if is_invalid_id(id) {
                total += id
            }
        }
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid_id_part2() {
        // Part 1 examples still work (exactly 2 repetitions)
        assert!(is_invalid_id(11)); // "1" x 2
        assert!(is_invalid_id(22)); // "2" x 2
        assert!(is_invalid_id(99)); // "9" x 2
        assert!(is_invalid_id(6464)); // "64" x 2
        assert!(is_invalid_id(123123)); // "123" x 2

        // New: 3+ repetitions
        assert!(is_invalid_id(111)); // "1" x 3
        assert!(is_invalid_id(999)); // "9" x 3
        assert!(is_invalid_id(1111)); // "1" x 4
        assert!(is_invalid_id(11111)); // "1" x 5

        // New: longer patterns repeated 3+ times
        assert!(is_invalid_id(123123123)); // "123" x 3
        assert!(is_invalid_id(1212121212)); // "12" x 5
        assert!(is_invalid_id(1111111)); // "1" x 7
        assert!(is_invalid_id(565656)); // "56" x 3
        assert!(is_invalid_id(824824824)); // "824" x 3
        assert!(is_invalid_id(2121212121)); // "2121" x 2 (actually "21" x 5)

        // From example - still one invalid
        assert!(is_invalid_id(1188511885)); // "11885" x 2
        assert!(is_invalid_id(222222)); // "2" x 6 or "22" x 3 or "222" x 2
        assert!(is_invalid_id(446446)); // "446" x 2
        assert!(is_invalid_id(38593859)); // "3859" x 2

        // Valid IDs (not repeated patterns)
        assert!(!is_invalid_id(1));
        assert!(!is_invalid_id(12));
        assert!(!is_invalid_id(123));
        assert!(!is_invalid_id(1234));
        assert!(!is_invalid_id(101));
        assert!(!is_invalid_id(1001));
    }

    #[test]
    fn test_example_part2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
                     1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
                     824824821-824824827,2121212118-2121212124";
        assert_eq!(solve(input), "4174379265");
    }

    #[test]
    fn test_pattern_detection() {
        // Test different pattern lengths and repetitions
        assert!(is_invalid_id(1111)); // pattern "1", len 1, repeat 4
        assert!(is_invalid_id(121212)); // pattern "12", len 2, repeat 3
        assert!(is_invalid_id(123123123)); // pattern "123", len 3, repeat 3
        assert!(is_invalid_id(12341234)); // pattern "1234", len 4, repeat 2

        // This should also match smaller patterns
        assert!(is_invalid_id(121212)); // could be "12" x 3 or "1212" x 1.5 (but 1.5 doesn't divide, so "12" x 3)
    }
}
