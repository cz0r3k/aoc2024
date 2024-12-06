use array2d::Array2D;
use day_06::*;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    println!("{output}");
}

fn part1(input: &str) -> String {
    let rows = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Field {
                        is_valid: true,
                        is_visited: false,
                    },
                    '#' => Field {
                        is_valid: false,
                        is_visited: false,
                    },
                    '^' => Field {
                        is_valid: true,
                        is_visited: true,
                    },
                    _ => Field {
                        is_valid: false,
                        is_visited: false,
                    },
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut map = Array2D::from_rows(&rows).unwrap();
    let start_position = map
        .enumerate_row_major()
        .filter(|(_, &field)| field.is_visited)
        .map(|((y, x), _)| (x as isize, y as isize))
        .next()
        .unwrap();
    let mut guard_position = start_position;
    let mut guard_direction = Direction::Up;
    loop {
        map[(guard_position.1 as usize, guard_position.0 as usize)].is_visited = true;

        let mut new_guard_position = new_position(guard_position, &guard_direction);
        let mut is_valid = false;

        while (0..map.column_len() as isize).contains(&new_guard_position.0)
            && (0..map.row_len() as isize).contains(&new_guard_position.1)
        {
            if map
                .get(new_guard_position.1 as usize, new_guard_position.0 as usize)
                .unwrap()
                .is_valid
            {
                is_valid = true;
                guard_position = new_guard_position;
                break;
            } else {
                guard_direction = next_direction(&guard_direction);
                new_guard_position = new_position(guard_position, &guard_direction);
            }
        }
        if !is_valid {
            break;
        }
    }
    map.as_row_major()
        .iter()
        .filter(|field| field.is_visited)
        .count()
        .to_string()
}
#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_test() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(part1(input), "41");
    }
}
