use day_09::Memory;
use std::iter::repeat;
use std::time::Instant;

fn main() {
    let input = include_str!("./input.txt");
    let now = Instant::now();
    let output = part2(input);
    let elapsed = now.elapsed();
    println!("Part 2: {:?}", elapsed);
    println!("{output}");
}

fn part2(input: &str) -> String {
    let digits = input
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize))
        .collect::<Vec<_>>();

    let mut flag = true;
    let mut id = 0;
    let mut memory = Vec::with_capacity(digits.len());

    for &size in digits.iter() {
        if flag {
            memory.push(Memory::File(id, size));
            id += 1;
        } else {
            memory.push(Memory::FreeSpace(size));
        }
        flag = !flag;
    }

    let mut ptr = memory.len() - 1;
    while ptr > 0 {
        if let Memory::File(_, file_size) = memory[ptr] {
            let mut left_ptr = 0;
            let mut free_space = None;
            while left_ptr < ptr{
                if let Memory::FreeSpace(free_size) = memory[left_ptr]{
                    if free_size >= file_size{
                        free_space = Some((left_ptr, free_size));
                        break;
                    }
                }
                left_ptr += 1;
            }
            if let Some((free_space_ptr, free_size)) = free_space {
                memory.swap(ptr, free_space_ptr);
                if file_size != free_size {
                    let diff = free_size - file_size;
                    let new_free = Memory::FreeSpace(diff);
                    memory.insert(left_ptr + 1, new_free);
                    memory[ptr + 1] = Memory::FreeSpace(file_size);
                }
            }
        }
        ptr -= 1;
    }
    //memory.iter().for_each(|memory|{print!("{}", Into::<String>::into(memory));});
    //println!();
    memory
        .iter()
        .flat_map(|&memory| {
            match memory{
                Memory::File(id, size) => {repeat(id).take(size)}
                Memory::FreeSpace(size) => {repeat(0).take(size)}

            }})
        .enumerate()
        .fold(0u64, |mut sum, (position, id)| {
            sum += position as u64 * id as u64;
            sum
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn part2_test() {
        let input = "2333133121414131402";
        assert_eq!(part2(input), "2858");
    }
}
