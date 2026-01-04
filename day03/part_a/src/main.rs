use aoc_utils::read_input;

fn main() {
    let input = read_input("../inputs/day03/input.txt");
    let answer = solve(&input);
    println!("{answer}");
}

fn find_max_joltage(bank: &str) -> u32 {
    let digits: Vec<u32> = bank.chars().filter_map(|c| c.to_digit(10)).collect();

    let mut max_joltage = 0;

    for i in 0..digits.len() {
        for j in (i + 1)..digits.len() {
            let joltage = digits[i] * 10 + digits[j];
            max_joltage = max_joltage.max(joltage);
        }
    }

    max_joltage
}

fn solve(input: &str) -> String {
    let input = input.trim_end();

    let total: u32 = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| find_max_joltage(line.trim()))
        .sum();

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(solve(input), "357");
    }

    #[test]
    fn test_individual_banks() {
        assert_eq!(find_max_joltage("987654321111111"), 98);
        assert_eq!(find_max_joltage("811111111111119"), 89);
        assert_eq!(find_max_joltage("234234234234278"), 78);
        assert_eq!(find_max_joltage("818181911112111"), 92);
    }
}
