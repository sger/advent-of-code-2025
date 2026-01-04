use aoc_utils::read_input;

fn main() {
    let input = read_input("../inputs/day05/input_b.txt");
    let answer = solve(&input);
    println!("{answer}");
}

fn solve(input: &str) -> String {
    let _ = input.trim_end();
    "TODO".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        assert_eq!(solve(""), "TODO");
    }
}
