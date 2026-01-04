use aoc_utils::read_input;

fn main() {
    let input = read_input("../inputs/day05/input.txt");
    let answer = solve(&input);
    println!("{answer}");
}

fn solve(input: &str) -> String {
    let input = input.trim_end();

    let sections: Vec<&str> = input.split("\n\n").collect();
    if sections.len() != 2 {
        return "0".to_string();
    }

    let fresh_ranges = parse_ranges(sections[0]);
    let available_ids = parse_ids(sections[1]);
    let fresh_count = count_fresh_ingredients(&fresh_ranges, &available_ids);

    fresh_count.to_string()
}

fn parse_ranges(section: &str) -> Vec<(i64, i64)> {
    section
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<i64> = line
                .split("-")
                .map(|n| n.trim().parse().expect("Invalid range number"))
                .collect();
            (parts[0], parts[1])
        })
        .collect()
}

fn parse_ids(section: &str) -> Vec<i64> {
    section
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim().parse().expect("Invalid ID"))
        .collect()
}

fn count_fresh_ingredients(ranges: &[(i64, i64)], ids: &[i64]) -> usize {
    ids.iter().filter(|&&id| is_id_fresh(id, ranges)).count()
}

fn is_id_fresh(id: i64, ranges: &[(i64, i64)]) -> bool {
    ranges.iter().any(|&(start, end)| id >= start && id <= end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(solve(input), "3");
    }

    #[test]
    fn test_parse_ranges() {
        let ranges = parse_ranges("3-5\n10-14\n16-20");
        assert_eq!(ranges, vec![(3, 5), (10, 14), (16, 20)]);
    }

    #[test]
    fn test_parse_ids() {
        let ids = parse_ids("1\n5\n8\n11");
        assert_eq!(ids, vec![1, 5, 8, 11]);
    }

    #[test]
    fn test_is_id_fresh() {
        let ranges = vec![(3, 5), (10, 14)];
        assert!(!is_id_fresh(1, &ranges));
        assert!(is_id_fresh(5, &ranges));
        assert!(!is_id_fresh(8, &ranges));
        assert!(is_id_fresh(11, &ranges));
    }
}
