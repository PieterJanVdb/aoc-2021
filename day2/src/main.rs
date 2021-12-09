use std::fs;

fn get_moves() -> Vec<(String, i32)> {
    fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .lines()
        .map(|s| {
            let (dir, amount) = s.split_at(s.find(' ').unwrap());
            (dir.to_string(), amount.trim().parse::<i32>().unwrap())
        })
        .collect()
}

fn calculate_position_1(moves: &[(String, i32)]) -> i32 {
    let (horizontal, depth) = moves.iter().fold((0, 0), |acc, x| match x.0.as_str() {
        "forward" => (acc.0 + x.1, acc.1),
        "down" => (acc.0, acc.1 + x.1),
        "up" => (acc.0, acc.1 - x.1),
        _ => acc,
    });

    horizontal * depth
}

fn calculate_position_2(moves: &[(String, i32)]) -> i32 {
    let (horizontal, depth, _) = moves.iter().fold((0, 0, 0), |acc, x| match x.0.as_str() {
        "forward" => (acc.0 + x.1, acc.1 + x.1 * acc.2, acc.2),
        "down" => (acc.0, acc.1, acc.2 + x.1),
        "up" => (acc.0, acc.1, acc.2 - x.1),
        _ => acc,
    });

    horizontal * depth
}

fn main() {
    let moves = get_moves();
    let result_1 = calculate_position_1(&moves);
    let result_2 = calculate_position_2(&moves);

    println!("Part 1: {}", result_1);
    println!("Part 2: {}", result_2);
}
