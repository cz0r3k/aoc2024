use std::collections::HashMap;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    println!("{output}");
}

fn part2(input: &str) -> String {
    let lines = input.lines();
    let mut left = HashMap::new();
    let mut right = HashMap::new();

    for line in lines {
        let mut line = line.split_whitespace();
        let left_value = line.next().unwrap().parse::<i32>().unwrap();
        let right_value = line.next().unwrap().parse::<i32>().unwrap();

        match left.get_mut(&left_value) {
            Some(v) => *v += 1,
            None => {
                left.insert(left_value, 1);
            }
        }
        match right.get_mut(&right_value) {
            Some(v) => *v += 1,
            None => {
                right.insert(right_value, 1);
            }
        }
    }

    left.iter()
        .map(|(k, v)| {
            k * v
                * match right.get(k) {
                    Some(value) => *value,
                    None => 0,
                }
        })
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn part2_test() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", part2(input));
    }
}
