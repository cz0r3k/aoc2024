use std::time::Instant;
use array2d::Array2D;
use day_06::*;

fn main() {
    let input = include_str!("./input2.txt");
    let now = Instant::now();
    let output = part2(input);
    let elapsed = now.elapsed();
    println!("Part 2: {:?}", elapsed);
    println!("{output}");
}

fn part2(input: &str) -> String {
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
        .as_row_major()
        .iter()
        .position(|field| field.is_visited)
        .unwrap();
    let start_position = (
        (start_position % map.row_len()) as isize,
        (start_position / map.row_len()) as isize,
    );
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

    let new_map = Array2D::from_rows(
        &map.as_rows()
            .iter()
            .map(|row| {
                row.iter()
                    .map(|field| NewField {
                        is_valid: field.is_valid,
                        directions: [false; 4],
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
    .unwrap();

    let mut sum = 0;
    map.enumerate_row_major()
        .filter(|(_, &field)| field.is_visited)
        .filter(|((y, x), _)| !(*x == start_position.0 as usize && *y == start_position.1 as usize))
        .map(|((y, x), _)| (y, x))
        .for_each(|(y_cord, x_cord)| {
            let mut new_map = new_map.clone();
            new_map[(y_cord, x_cord)].is_valid = false;
            guard_position = start_position;
            let mut guard_direction = Direction::Up;
            loop {
                if new_map[(guard_position.1 as usize, guard_position.0 as usize)].directions
                    [direction_to_usize(&guard_direction)]
                {
                    sum += 1;
                    break;
                }
                new_map[(guard_position.1 as usize, guard_position.0 as usize)].directions
                    [direction_to_usize(&guard_direction)] = true;

                let mut new_guard_position = new_position(guard_position, &guard_direction);
                let mut is_valid = false;
                let mut counter = 0;

                while (0..new_map.column_len() as isize).contains(&new_guard_position.0)
                    && (0..new_map.row_len() as isize).contains(&new_guard_position.1)
                {
                    if new_map
                        .get(new_guard_position.1 as usize, new_guard_position.0 as usize)
                        .unwrap()
                        .is_valid
                    {
                        is_valid = true;
                        guard_position = new_guard_position;
                        break;
                    } else {
                        if counter == 4 {
                            sum += 1;
                            break;
                        }
                        guard_direction = next_direction(&guard_direction);
                        new_guard_position = new_position(guard_position, &guard_direction);
                        counter += 1;
                    }
                }
                if !is_valid {
                    break;
                }
            }
        });
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn part2_test() {
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
        assert_eq!(part2(input), "6");
    }
}
