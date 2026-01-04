use aoc_utils::read_input;

fn main() {
    let input = read_input("../inputs/day04/input.txt");
    let answer = solve(&input);
    println!("{answer}");
}

fn solve(input: &str) -> String {
    let mut grid: Vec<Vec<char>> = input
        .trim_end()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    if grid.is_empty() {
        return "0".to_string();
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let mut total_removed = 0;

    loop {
        let accessible: Vec<(usize, usize)> = (0..rows)
            .flat_map(|row| (0..cols).map(move |col| (row, col)))
            .filter(|&(row, col)| grid[row][col] == '@')
            .filter(|&(row, col)| count_adjacent_rolls(&grid, row, col, rows, cols) < 4)
            .collect();

        if accessible.is_empty() {
            break;
        }

        for (row, col) in &accessible {
            grid[*row][*col] = '.';
        }

        total_removed += accessible.len();
    }

    total_removed.to_string()
}

fn count_adjacent_rolls(
    grid: &[Vec<char>],
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
) -> usize {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    directions
        .iter()
        .filter_map(|&(dr, dc)| {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
                Some((new_row as usize, new_col as usize))
            } else {
                None
            }
        })
        .filter(|&(r, c)| grid[r][c] == '@')
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        let input = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!(solve(input), "43");
    }
}
