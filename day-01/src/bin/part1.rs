fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    println!("{output}");
}

fn part1(input: &str) -> String {
    let lines = input.lines();
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in lines {
        let mut line = line.split_whitespace();
        left.push(line.next().unwrap().parse::<i32>().unwrap());
        right.push(line.next().unwrap().parse::<i32>().unwrap());
    }
    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_test() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("11", part1(input));
    }
}
