fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    println!("{output}");
}

fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn part2_test() {
        let input = "";
        assert_eq!(part2(input), "");
    }
}
