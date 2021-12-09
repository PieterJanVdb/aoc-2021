use std::fs;

fn get_moves() -> Vec<(String, i32)> {
    fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .lines()
        .map(|s| {
            let (op, amount) = s.split_at(s.find(' ').unwrap());
            (op.to_string(), amount.trim().parse::<i32>().unwrap())
        })
        .collect()
}

fn calculate_position(moves: &[(String, i32)]) -> (i32, i32) {
    let (horizontal, depth, aim) =
        moves
            .iter()
            .fold((0, 0, 0), |(horizontal, depth, aim), (op, n)| {
                match op.as_str() {
                    "forward" => (horizontal + n, depth + n * aim, aim),
                    "down" => (horizontal, depth, aim + n),
                    "up" => (horizontal, depth, aim - n),
                    _ => (horizontal, depth, aim),
                }
            });

    // aim is masquerading as depth of part 1 in this case
    (horizontal * aim, horizontal * depth)
}

fn main() {
    let moves = get_moves();
    let (result_1, result_2) = calculate_position(&moves);

    println!("Part 1: {}", result_1);
    println!("Part 2: {}", result_2);
}
