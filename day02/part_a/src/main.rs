use aoc_utils::read_input;

fn main() {
    let input = read_input("../inputs/day02/input_a.txt");
    let answer = solve(&input);
    println!("{answer}");
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

fn is_invalid_id(n: u64) -> bool {
    let s = n.to_string(); // Convert to string: "1010"
    let len = s.len(); // Get length: 4

    if len % 2 != 0 {
        // Is 4 odd no it's even
        return false;
    }

    let mid = len / 2; // Find middle: 4 / 2 = 2
    &s[0..mid] == &s[mid..] // Compare "10" == "10" -> true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_invalid_id() {
        assert!(is_invalid_id(55)); // "5" repeated
        assert!(is_invalid_id(6464)); // "64" repeated
        assert!(is_invalid_id(123123)); // "123" repeated
        assert!(!is_invalid_id(101)); // Odd length, can't be repeated

        // From the example ranges
        assert!(is_invalid_id(11));
        assert!(is_invalid_id(22));
        assert!(is_invalid_id(99));
        assert!(is_invalid_id(1010));
        assert!(is_invalid_id(1188511885));
        assert!(is_invalid_id(222222));
        assert!(is_invalid_id(446446));
        assert!(is_invalid_id(38593859));

        // Valid IDs (not repeated patterns)
        assert!(!is_invalid_id(12));
        assert!(!is_invalid_id(100));
        assert!(!is_invalid_id(1234));
    }

    #[test]
    fn test_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
                     1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
                     824824821-824824827,2121212118-2121212124";
        assert_eq!(solve(input), "1227775554");
    }
}
