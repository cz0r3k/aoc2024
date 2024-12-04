use array2d::Array2D;
use std::str;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    println!("{output}");
}
fn part1(input: &str) -> String {
    let rows = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let array = Array2D::from_columns(&rows).unwrap();
    (count_vertically(&array, "XMAS", "SAMX")
        + count_horizontally(&array, "XMAS", "SAMX")
        + count_diagonally(&array, "XMAS", "SAMX"))
    .to_string()
}

fn count_horizontally(array: &Array2D<char>, patter1: &str, pattern2: &str) -> usize {
    array
        .rows_iter()
        .map(|row| count_in_line(&row.collect::<String>(), patter1, pattern2))
        .sum()
}

fn count_vertically(array: &Array2D<char>, patter1: &str, pattern2: &str) -> usize {
    array
        .columns_iter()
        .map(|column| count_in_line(&column.collect::<String>(), patter1, pattern2))
        .sum()
}

fn count_diagonally(array: &Array2D<char>, patter1: &str, pattern2: &str) -> usize {
    let mut diagonals1: Vec<Vec<char>> = vec![Vec::new(); array.column_len() + array.row_len() - 1];
    let mut diagonals2: Vec<Vec<char>> = vec![Vec::new(); array.column_len() + array.row_len() - 1];
    for i in 0..array.column_len() {
        for j in 0..array.row_len() {
            diagonals1[i + j].push(*array.get(j, i).unwrap());
            let index = i + array.row_len() - j - 1;
            diagonals2[index].push(*array.get(j, i).unwrap());
        }
    }
    diagonals1
        .iter()
        .map(|diag| count_in_line(&diag.iter().collect::<String>(), patter1, pattern2))
        .sum::<usize>()
        + diagonals2
            .iter()
            .map(|diag| count_in_line(&diag.iter().collect::<String>(), patter1, pattern2))
            .sum::<usize>()
}

fn count_in_line(line: &str, patter1: &str, pattern2: &str) -> usize {
    line.as_bytes()
        .windows(patter1.len())
        .filter(|&w| w == patter1.as_bytes() || w == pattern2.as_bytes())
        .count()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_test() {
        let input = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";
        assert_eq!(part1(input), "3");
    }
    #[test]
    fn part1_test2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(part1(input), "18");
    }
}
