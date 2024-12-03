#![feature(iter_map_windows)]
#![feature(test)]

extern crate test;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    println!("{output}");
}

fn part2n2(input: &str) -> String {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let nums = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let diff = nums
            .iter()
            .map_windows(|[&x, &y]| x - y)
            .collect::<Vec<_>>();
        if diff.iter().all(|x| (1..=3).contains(x)) || diff.iter().all(|x| (-3..0).contains(x)) {
            sum += 1;
            continue;
        }
        for i in 0..nums.len() {
            let mut new_nums = nums.clone();
            new_nums.remove(i);
            let diff = new_nums
                .iter()
                .map_windows(|[&x, &y]| x - y)
                .collect::<Vec<_>>();
            if diff.iter().all(|x| (1..=3).contains(x)) || diff.iter().all(|x| (-3..0).contains(x))
            {
                sum += 1;
                break;
            }
        }
    }
    sum.to_string()
}

fn part2(input: &str) -> String {
    let lines = input.lines();
    let mut sum = 0;
    for line in lines {
        let nums = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut diff = nums
            .iter()
            .map_windows(|[&x, &y]| x - y)
            .collect::<Vec<_>>();
        if diff.iter().all(|x| (1..=3).contains(x)) || diff.iter().all(|x| (-3..0).contains(x)) {
            sum += 1;
            continue;
        }
        let up = diff.iter().filter(|&x| *x < 0).count();
        let down = diff.iter().filter(|&x| *x > 0).count();
        let zero = diff.iter().filter(|&x| *x == 0).count();
        let index = match (up, down, zero) {
            (1, _, 0) => diff.iter().position(|x| *x < 0).unwrap(),
            (_, 1, 0) => diff.iter().position(|x| *x > 0).unwrap(),
            (0, _, 1) => diff.iter().position(|x| *x == 0).unwrap(),
            (_, 0, 1) => diff.iter().position(|x| *x == 0).unwrap(),
            (_, 0, 0) => diff.iter().position(|x| *x < -3).unwrap(),
            (0, _, 0) => diff.iter().position(|x| *x > 3).unwrap(),
            _ => continue,
        };

        if up > 1 {
            diff = diff.iter().map(|x| x * -1).collect();
        }

        let left = if index == 0 {
            None
        } else {
            diff.get(index - 1).cloned()
        };
        let right = diff.get(index + 1).cloned();
        let value = diff[index];

        match (left, right) {
            (None, Some(right)) => {
                if (1..=3).contains(&(right + value)) {
                    diff[index + 1] = diff[index] + diff[index + 1];
                }
            }
            (Some(left), None) => {
                if (1..=3).contains(&(left + value)) {
                    diff[index - 1] = diff[index] + diff[index - 1];
                }
            }
            (Some(left), Some(right)) => {
                if left > 3 {
                    diff[index - 1] += diff[index];
                } else if right > 3 {
                    diff[index + 1] += diff[index];
                } else if (1..=3).contains(&(right + value)) {
                    diff[index + 1] += diff[index];
                } else if (1..=3).contains(&(left + value)) {
                    diff[index - 1] += diff[index];
                } else {
                    continue;
                }
            }
            (None, None) => {}
        }
        diff.remove(index);

        if diff.iter().all(|x| (1..=3).contains(x)) {
            sum += 1;
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use crate::{part2, part2n2};

    #[test]
    fn part2_test() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part2(input), "4");
    }
    #[test]
    fn part2_test2() {
        let input = "75 77 72 70 69
1 5 6
1 2 6 7
52 52 51 52 52
2 4 3 1
3 1 2 4 5 6
1 2 20";
        assert_eq!(part2(input), "5");
    }
    #[test]
    fn part2_test3() {
        let input = "57 56 57 59 60 63 64 65
91 92 95 93 94
16 13 15 13 12 11 9 6
40 41 43 44 47 46 47 49
1 1 4 5 7 8
3 6 4 8
1 9 2 3
5 7 6 5";
        assert_eq!(part2(input), "8");
    }

    #[test]
    fn part2_test4() {
        let input = "5 4 3 2 1 2
4 5 4 3 2 1
1 1 2 3 4
1 2 3 3 4
1 2 3 4 4
4 4 3 2 1
4 3 3 2 1
4 3 2 1 1
1 2 6 4 5
6 2 3 4 5
1 2 3 4 9
5 4 3 2 6
1 2 3 4 9
9 2 3 4 5
1 2 3 500 4
1 2 3 0 4
1 9 10 11";
        assert_eq!(part2(input), "17");
    }

    #[test]
    fn part2_test5() {
        let input = "9 2 3 4 5
1 9 3 4 5
1 2 9 4 5
1 2 3 9 5
1 2 3 4 9 ";
        assert_eq!(part2(input), "5");
    }

    #[test]
    fn part2_test6() {
        let input = "13 9 11 8 7
8 12 11 14 17 20";
        assert_eq!(part2(input), "2");
    }

    #[bench]
    fn part2_bench(b: &mut Bencher) {
        let input = include_str!("./input2.txt");
        b.iter(|| part2(input));
    }

    #[bench]
    fn part2n2_bench(b: &mut Bencher) {
        let input = include_str!("./input2.txt");
        b.iter(|| part2n2(input));
    }
}
