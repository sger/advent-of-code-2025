use aoc_utils::read_input;

fn main() {
    let input = read_input("../inputs/day03/input.txt");
    let answer = solve(&input);
    println!("{answer}");
}

fn find_max_k_digits(s: &str, k: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let to_remove = n - k;

    let mut result = Vec::new();
    let mut remove_count = 0;

    for i in 0..n {
        while remove_count < to_remove && !result.is_empty() && result.last().unwrap() < &chars[i] {
            result.pop();
            remove_count += 1;
        }
        result.push(chars[i]);
    }

    result.truncate(k);

    result.iter().collect()
}

fn solve(input: &str) -> String {
    let total: i64 = input
        .trim_end()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let max_joltage = find_max_k_digits(line.trim(), 12);
            max_joltage.parse::<i64>().unwrap()
        })
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
        assert_eq!(solve(input), "3121910778619");
    }

    #[test]
    fn test_find_max_k_digits() {
        assert_eq!(find_max_k_digits("987654321111111", 12), "987654321111");
        assert_eq!(find_max_k_digits("811111111111119", 12), "811111111119");
        assert_eq!(find_max_k_digits("234234234234278", 12), "434234234278");
        assert_eq!(find_max_k_digits("818181911112111", 12), "888911112111");
    }
}
