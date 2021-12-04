use std::fs;

fn get_input() -> Vec<i32> {
    fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn part1(input: &[i32]) -> usize {
    input.windows(2).filter(|x| x[1] > x[0]).count()
}

fn part2(input: &[i32]) -> usize {
    input.windows(4).filter(|x| x[3] > x[0]).count()
}

fn main() {
    let input = get_input();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
