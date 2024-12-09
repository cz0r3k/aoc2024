use day_09::Memory;
use std::iter::repeat;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    println!("{output}");
}

fn part1(input: &str) -> String {
    let digits = input
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize))
        .collect::<Vec<_>>();

    let mut flag = true;
    let mut id = 0;
    let mut memory = Vec::with_capacity(digits.len());
    let mut occupied_memory = 0usize;

    for digit in digits {
        if flag {
            memory.push(Memory::File(id, digit));
            occupied_memory += digit;
            id += 1;
        } else {
            memory.push(Memory::FreeSpace(digit));
        }
        flag = !flag;
    }

    //memory.iter().for_each(|memory|{print!("{}", Into::<String>::into(memory));});
    //println!();

    let mut new_memory = Vec::with_capacity(occupied_memory);
    let mut left_ptr = 0;
    let mut right_ptr = memory.len() - 1;
    while left_ptr <= right_ptr {
        if matches!(memory[right_ptr], Memory::FreeSpace(_)) {
            right_ptr -= 1;
            continue;
        }
        match memory[left_ptr] {
            Memory::File(id, size) => {
                new_memory.extend_from_slice(&repeat(id).take(size).collect::<Vec<_>>());
                left_ptr += 1;
            }
            Memory::FreeSpace(free_space_size) => {
                if let Memory::File(id, file_size) = memory[right_ptr] {
                    match free_space_size.cmp(&file_size) {
                        std::cmp::Ordering::Equal => {
                            new_memory
                                .extend_from_slice(&repeat(id).take(file_size).collect::<Vec<_>>());
                            left_ptr += 1;
                            right_ptr -= 1;
                        }
                        std::cmp::Ordering::Less => {
                            new_memory.extend_from_slice(
                                &repeat(id).take(free_space_size).collect::<Vec<_>>(),
                            );
                            memory[right_ptr] = Memory::File(id, file_size - free_space_size);
                            left_ptr += 1;
                        }
                        std::cmp::Ordering::Greater => {
                            new_memory
                                .extend_from_slice(&repeat(id).take(file_size).collect::<Vec<_>>());
                            memory[left_ptr] = Memory::FreeSpace(free_space_size - file_size);
                            right_ptr -= 1;
                        }
                    }
                }
            }
        }
    }
    //new_memory.iter().for_each(|memory|{print!("{}", memory);});
    //println!();
    new_memory
        .iter()
        .enumerate()
        .fold(0u64, |mut sum, (position, id)| {
            sum += position as u64 * *id as u64;
            sum
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_test() {
        let input = "2333133121414131402";
        assert_eq!(part1(input), "1928");
    }
}
