use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    println!("{output}");
}

fn part1(input: &str) -> String {
    let lines = input.lines();
    let mut front = HashMap::<u32, Vec<u32>>::new();
    let mut back = HashSet::<u32>::new();

    let rules = lines.clone().take_while(|&line| !line.is_empty());
    let updates = lines.skip_while(|&line| !line.is_empty()).skip(1);

    rules
        .map(|line| {
            let mut nums = line.split('|');
            (
                nums.next().unwrap().parse::<u32>().unwrap(),
                nums.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .for_each(|(first, second)| {
            front
                .entry(first)
                .and_modify(|key| key.push(second))
                .or_insert(vec![second]);
            back.insert(second);
        });
    let correct_updates = updates
        .map(|update| {
            update
                .split(',')
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|pages| {
            let mut first = Vec::<u32>::new();
            let mut second = Vec::<u32>::new();
            for page in pages {
                if front.contains_key(page) {
                    if front[page].iter().any(|num| second.contains(num)) {
                        return false;
                    }
                    first.push(*page);
                }
                if back.contains(page) {
                    second.push(*page);
                }
            }
            true
        })
        .collect::<Vec<Vec<u32>>>();
    correct_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_test() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(part1(input), "143");
    }
}
