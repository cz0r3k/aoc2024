use std::time::Instant;

fn main() {
    let input = include_str!("./input.txt");
    let now = Instant::now();
    let output = part2(input);
    let elapsed = now.elapsed();
    println!("Part 2: {:?}", elapsed);
    println!("{output}");
}

fn is_valid(target: i64, numbers: &[i64], index: isize) -> bool {
    if index == -1 {
        return target == 0;
    }
    if target == 0 {
        return false;
    }
    let number = numbers[index as usize];
    let ten_power = 10i64.pow(number.ilog10() + 1);

    (if target % number == 0 {
        is_valid(target / number, numbers, index - 1)
    } else {
        false
    } || if target - number >= 0 {
        is_valid(target - number, numbers, index - 1)
    } else {
        false
    } || if target % ten_power == number {
        is_valid(target / ten_power, numbers, index - 1)
    } else {
        false
    })
}

fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(target, numbers)| {
            (
                target.parse::<i64>().unwrap(),
                numbers
                    .split_whitespace()
                    .map(|number| number.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(target, numbers)| {
            if is_valid(target, &numbers, numbers.len() as isize - 1) {
                target
            } else {
                0
            }
        })
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn part2_test() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(part2(input), "11387");
    }
}
