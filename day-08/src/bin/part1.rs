use day_08::Field;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    println!("{output}");
}

fn part1(input: &str) -> String {
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let mut map = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => row.push(Field {
                    antenna: None,
                    anti_node: false,
                }),
                _ => {
                    antennas
                        .entry(c)
                        .or_default()
                        .push((y as isize, x as isize));
                    row.push(Field {
                        antenna: Some(c),
                        anti_node: false,
                    });
                }
            }
        }
        map.push(row);
    }

    let row_len = map.len() as isize;
    let column_len = map[0].len() as isize;

    let is_valid =
        |(x, y): (isize, isize)| (0..column_len).contains(&x) && (0..row_len).contains(&y);

    let get_vector = |&(x1, y1): &(isize, isize), &(x2, y2): &(isize, isize)| (x2 - x1, y2 - y1);
    let add_vector = |&(x, y): &(isize, isize), &(vector_x, vector_y): &(isize, isize)| {
        (x + vector_x, y + vector_y)
    };
    let reverse_vector = |&(x, y): &(isize, isize)| (-x, -y);

    let mut counter = 0;
    antennas.iter().for_each(|(_, frequency)| {
        frequency.iter().combinations(2).for_each(|pair| {
            let (first_antenna, second_antenna) = pair.into_iter().collect_tuple().unwrap();

            let vector = get_vector(first_antenna, second_antenna);
            let second_vector = reverse_vector(&vector);

            let first_antinode = add_vector(second_antenna, &vector);
            let second_antinode = add_vector(first_antenna, &second_vector);

            if is_valid(first_antinode)
                && !map[first_antinode.0 as usize][first_antinode.1 as usize].anti_node
            {
                map[first_antinode.0 as usize][first_antinode.1 as usize].anti_node = true;
                counter += 1;
            }
            if is_valid(second_antinode)
                && !map[second_antinode.0 as usize][second_antinode.1 as usize].anti_node
            {
                map[second_antinode.0 as usize][second_antinode.1 as usize].anti_node = true;
                counter += 1;
            }
        })
    });
    //print_map(&map);
    counter.to_string()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_test() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(part1(input), "14");
    }
}
