use std::fs;

fn get_input() -> Vec<i32> {
    fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn get_count(input: &[i32], size: usize) -> usize {
    input.windows(size).filter(|x| x[size - 1] > x[0]).count()
}

fn main() {
    let input = get_input();

    println!("Part 1: {}", get_count(&input, 2));
    println!("Part 2: {}", get_count(&input, 4));
}
