use aoc_utils::read_input;

fn main() {
    let input = read_input("../inputs/day05/input.txt");
    let answer = solve(&input);
    println!("{answer}");
}

fn solve(input: &str) -> String {
    let input = input.trim_end();

    let ranges_section = input.split("\n\n").next().unwrap_or("");

    let fresh_ranges = parse_ranges(ranges_section);

    let total_fresh_ids = count_total_fresh_ids(&fresh_ranges);

    total_fresh_ids.to_string()
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

fn count_total_fresh_ids(ranges: &[(i64, i64)]) -> usize {
    if ranges.is_empty() {
        return 0;
    }

    let merged = merge_ranges(ranges);

    merged
        .iter()
        .map(|(start, end)| (end - start + 1) as usize)
        .sum()
}

fn merge_ranges(ranges: &[(i64, i64)]) -> Vec<(i64, i64)> {
    if ranges.is_empty() {
        return vec![];
    }

    let mut sorted_ranges = ranges.to_vec();
    sorted_ranges.sort_by_key(|&(start, _)| start);

    let mut merged = vec![sorted_ranges[0]];

    for &(start, end) in &sorted_ranges[1..] {
        let last_idx = merged.len() - 1;
        let (last_start, last_end) = merged[last_idx];

        if start <= last_end + 1 {
            merged[last_idx] = (last_start, last_end.max(end));
        } else {
            merged.push((start, end));
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
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
        assert_eq!(solve(input), "14");
    }

    #[test]
    fn test_merge_ranges() {
        // Overlapping ranges
        let ranges = vec![(3, 5), (10, 14), (12, 18), (16, 20)];
        let merged = merge_ranges(&ranges);
        assert_eq!(merged, vec![(3, 5), (10, 20)]);

        // Adjacent ranges
        let ranges2 = vec![(1, 3), (4, 6), (7, 9)];
        let merged2 = merge_ranges(&ranges2);
        assert_eq!(merged2, vec![(1, 9)]);

        // Non-overlapping ranges
        let ranges3 = vec![(1, 3), (5, 7), (9, 11)];
        let merged3 = merge_ranges(&ranges3);
        assert_eq!(merged3, vec![(1, 3), (5, 7), (9, 11)]);
    }

    #[test]
    fn test_count_total_fresh_ids() {
        let ranges = vec![(3, 5), (10, 20)];
        assert_eq!(count_total_fresh_ids(&ranges), 14);
    }

    #[test]
    fn test_parse_ranges() {
        let ranges = parse_ranges("3-5\n10-14\n16-20");
        assert_eq!(ranges, vec![(3, 5), (10, 14), (16, 20)]);
    }
}
