use array2d::Array2D;

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const ALL_DIRECTIONS: [(isize, isize); 9] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
];

const CORNERS: [[bool; 3]; 3] = [
    [true, false, true],
    [false, true, false],
    [false, false, false],
];
#[derive(Copy, Clone)]
struct Field {
    sign: char,
    is_marked: bool,
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    println!("{output}");
}

fn mark_region(map: &mut Array2D<Field>, cord: (isize, isize)) -> (usize, usize) {
    if map[(cord.0 as usize, cord.1 as usize)].is_marked {
        return (0, 0);
    }
    map[(cord.0 as usize, cord.1 as usize)].is_marked = true;
    let is_valid = |y: &isize, x: &isize| {
        (0..map.row_len() as isize).contains(y) && (0..map.column_len() as isize).contains(x)
    };
    let neighbours = DIRECTIONS
        .iter()
        .map(|(y, x)| (y + cord.0, x + cord.1))
        .filter(|(y, x)| is_valid(y, x))
        .filter(|(y, x)| {
            map[(*y as usize, *x as usize)].sign == map[(cord.0 as usize, cord.1 as usize)].sign
        })
        .collect::<Vec<_>>();

    if neighbours.is_empty() {
        return (1, 4);
    }

    let all_neighbours = ALL_DIRECTIONS
        .iter()
        .map(|(y, x)| (y + cord.0, x + cord.1))
        .map(|(y, x)| {
            if is_valid(&y, &x) {
                map[(y as usize, x as usize)].sign == map[(cord.0 as usize, cord.1 as usize)].sign
            } else {
                false
            }
        })
        .collect::<Vec<_>>();

    let corners = (0..4)
        .map(|i| {
            CORNERS.contains(<&[bool; 3]>::try_from(&all_neighbours[i * 2..i * 2 + 3]).unwrap())
        })
        .filter(|c| *c)
        .count();

    neighbours
        .iter()
        .map(|(y, x)| mark_region(map, (*y, *x)))
        .fold(
            (1, corners),
            |(sum_fields, sum_corners), (new_fields, new_corners)| {
                (sum_fields + new_fields, sum_corners + new_corners)
            },
        )
}

fn part2(input: &str) -> String {
    let mut map = Array2D::from_rows(
        &input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| Field {
                        sign: c,
                        is_marked: false,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
    .unwrap();

    let mut sum = 0;
    for y in 0..map.row_len() {
        for x in 0..map.column_len() {
            if !map[(y, x)].is_marked {
                let (fields, corners) = mark_region(&mut map, (y as isize, x as isize));
                sum += fields * corners;
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
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(part2(input), "1206");
    }
}
