use aoc_utils::read_input;

fn main() {
    let input = read_input("../inputs/day01/input_a.txt");
    let answer = solve(&input);
    println!("{answer}");
}

fn solve(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut i = 0usize;

    let mut position: i32 = 50;
    let mut zero_count: u32 = 0;

    while i < bytes.len() {
        // Skip whitespace / empty lines
        while i < bytes.len()
            && (bytes[i] == b'\n' || bytes[i] == b'\r' || bytes[i] == b' ' || bytes[i] == b'\t')
        {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }

        // Direction
        let dir = bytes[i];
        i += 1;

        // Skip whitespace between dir and number
        while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t') {
            i += 1;
        }

        // Parse distance
        let mut dist: i32 = 0;
        let mut saw_digit = false;
        while i < bytes.len() {
            let c = bytes[i];
            if (b'0'..=b'9').contains(&c) {
                saw_digit = true;
                dist = dist * 10 + (c - b'0') as i32;
                i += 1;
            } else {
                break;
            }
        }

        if !saw_digit {
            panic!("Invalid or missing distance near byte index {i}");
        }

        // Skip to end of line
        while i < bytes.len() && bytes[i] != b'\n' {
            i += 1;
        }
        if i < bytes.len() {
            i += 1;
        }

        // Apply rotation
        match dir {
            b'L' => position -= dist,
            b'R' => position += dist,
            _ => panic!("Invalid direction byte: {}", dir),
        }

        position = position.rem_euclid(100);

        if position == 0 {
            zero_count += 1;
        }
    }

    zero_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_from_prompt() {
        let input = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        assert_eq!(solve(input), "3");
    }

    #[test]
    fn wraparound_left_and_right() {
        assert_eq!(solve("L50\n"), "1");
        assert_eq!(solve("R50\n"), "1");
        assert_eq!(solve("L50\nL1\nR1\n"), "2");
    }

    #[test]
    fn ignores_blank_lines_and_crlf() {
        let input = "R50\r\n\r\nL1\r\n";
        assert_eq!(solve(input), "1");
    }
}
