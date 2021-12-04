use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");

    let result = input
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(3)
        .map(|x| x.iter().sum::<i32>())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|p| p[0] < p[1])
        .count();

    println!("There are {} measurements larger than the previous", result);
}
