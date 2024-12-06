use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    println!("{output}");
}

fn part2(input: &str) -> String {
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
    let incorrect_updates = updates
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
                        return true;
                    }
                    first.push(*page);
                }
                if back.contains(page) {
                    second.push(*page);
                }
            }
            false
        });
    incorrect_updates
        .map(|update| {
            let mut order = HashMap::<u32, (u32, Vec<u32>)>::new();
            let mut is_node_has_entrypoint = HashMap::<u32, bool>::new();
            update.iter().for_each(|page| {
                is_node_has_entrypoint.entry(*page).or_insert(false);
                let next_pages = front
                    .get(page)
                    .unwrap_or(&vec![])
                    .iter()
                    .filter(|&x| update.contains(x))
                    .copied()
                    .collect::<Vec<u32>>();
                next_pages.iter().for_each(|&page| {
                    is_node_has_entrypoint.insert(page, true);
                });
                order
                    .entry(*page)
                    .and_modify(|(_, v)| *v = next_pages.clone())
                    .or_insert((0, next_pages.clone()));
            });
            let (&start_node, _) = is_node_has_entrypoint.iter().find(|(_, &val)| !val).unwrap();
            let mut nodes = vec![start_node];

            while let Some(node) = nodes.pop() {
                let new_cost = order.get(&node).unwrap().0 + 1;
                let next_pages = order.get(&node).unwrap().1.clone();
                for next_page in next_pages {
                    order.entry(next_page).and_modify(|(cost, _)| {
                        if new_cost > *cost {
                            *cost = new_cost;
                            nodes.push(next_page);
                        }
                    });
                }
            }
            let (&middle_page, _) = order.iter().find(|(_, &(cost, _))| cost == (update.len() / 2) as u32).unwrap();
            middle_page
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn part2_test() {
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
        assert_eq!(part2(input), "123");
    }
}
