use regex::Regex;
fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    println!("{output}");
}

fn part1(input: &str) -> String {
    Regex::new(r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)")
        .unwrap()
        .captures_iter(input)
        .map(|caps| {
            caps.name("first").unwrap().as_str().parse::<i32>().unwrap()
                * caps
                    .name("second")
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap()
        })
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_test() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(input), "161");
    }
}
