use array2d::Array2D;

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

#[derive(Copy, Clone)]
struct Field {
    sign: char,
    is_marked: bool,
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
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
    neighbours
        .iter()
        .map(|(y, x)| mark_region(map, (*y, *x)))
        .fold(
            (1, 4 - neighbours.len()),
            |(sum_fields, sum_fences), (fields, fences)| (sum_fields + fields, sum_fences + fences),
        )
}

fn part1(input: &str) -> String {
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
                let (fields, fences) = mark_region(&mut map, (y as isize, x as isize));
                sum += fields * fences;
            }
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_test() {
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
        assert_eq!(part1(input), "1930");
    }
}
