use regex::{Regex, RegexBuilder};

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    println!("{output}");
}

fn part2(input: &str) -> String {
    let input = &input.lines().collect::<Vec<&str>>().join("x");
    let removed = RegexBuilder::new(r"(don't\(\).+do\(\))")
        .swap_greed(true)
        .build()
        .unwrap()
        .replace_all(input, "x")
        .to_string();
    let removed = Regex::new(r"(don't\(\).+)")
        .unwrap()
        .replace_all(&removed, "x")
        .to_string();

    Regex::new(r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)")
        .unwrap()
        .captures_iter(&removed)
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
    use crate::part2;

    #[test]
    fn part2_test() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(input), "48");
    }
}
