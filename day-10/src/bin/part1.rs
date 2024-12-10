use array2d::Array2D;
const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    println!("{output}");
}

fn get_neighbors(map: &Array2D<(bool, i32)>, (y, x): &(usize, usize)) -> Vec<(usize, usize)> {
    let width = 0..map.column_len() as isize;
    let height = 0..map.row_len() as isize;
    DIRECTIONS
        .iter()
        .map(|&(dy, dx)| (*y as isize + dy, *x as isize + dx))
        .filter(|(y, x)| width.contains(x) && height.contains(y))
        .map(|(y, x)| (y as usize, x as usize))
        .collect()
}
fn mark_possible_neighbors(map: &mut Array2D<(bool, i32)>, cord: (usize, usize)) {
    map[cord].0 = true;
    let height = map[cord].1;
    let neighbors = get_neighbors(map, &cord);
    for n_cord in neighbors {
        if !map[n_cord].0 && map[n_cord].1 - height == 1 {
            mark_possible_neighbors(map, n_cord);
        }
    }
}

fn part1(input: &str) -> String {
    let map = Array2D::from_rows(
        &input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| (false, c.to_digit(10).unwrap() as i32))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
    .unwrap();

    let trailheads = map
        .enumerate_row_major()
        .filter(|(_, (_, height))| *height == 0)
        .map(|((y, x), _)| (y, x))
        .collect::<Vec<_>>();

    trailheads
        .iter()
        .map(|&cord| {
            let mut new_map = map.clone();
            mark_possible_neighbors(&mut new_map, cord);
            new_map
                .elements_row_major_iter()
                .filter(|(is_possible, height)| *is_possible && *height == 9)
                .count()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_test() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(part1(input), "36");
    }
}
