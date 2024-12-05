use array2d::Array2D;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    println!("{output}");
}

fn part2(input: &str) -> String {
    let rows = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let array = Array2D::from_columns(&rows).unwrap();

    let mut sum = 0;
    for i in 1..array.column_len() - 1 {
        for j in 1..array.row_len() - 1 {
            if *array.get(i, j).unwrap() == 'A' {
                let table = [
                    *array.get(i - 1, j - 1).unwrap() == 'M'
                        && *array.get(i + 1, j + 1).unwrap() == 'S',
                    *array.get(i - 1, j - 1).unwrap() == 'S'
                        && *array.get(i + 1, j + 1).unwrap() == 'M',
                    *array.get(i + 1, j - 1).unwrap() == 'M'
                        && *array.get(i - 1, j + 1).unwrap() == 'S',
                    *array.get(i + 1, j - 1).unwrap() == 'S'
                        && *array.get(i - 1, j + 1).unwrap() == 'M',
                ];
                if table.iter().filter(|&x| *x).count() == 2 {
                    sum += 1;
                }
            }
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn part2_test() {
        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        assert_eq!(part2(input), "9");
    }
}
