fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    println!("{output}");
}

fn part1(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_test() {
        let input = "";
        assert_eq!(part1(input), "");
    }
}
