#![feature(iter_map_windows)]

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    println!("{output}");
}

fn part1(input: &str) -> String {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let nums = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        let diff = nums.map_windows(|[x, y]| x - y).collect::<Vec<_>>();
        if diff.iter().all(|x| (1..=3).contains(x)) || diff.iter().all(|x| (-3..0).contains(x)) {
            sum += 1;
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_test() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part1(input), "2");
    }
}
