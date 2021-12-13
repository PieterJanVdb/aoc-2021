use std::fs;

fn get_input() -> Vec<Vec<u32>> {
    fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .lines()
        .map(|s| {
            s.trim()
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn filter(input: &[Vec<u32>], index: usize, decider: fn(usize, usize) -> bool) -> u32 {
    if input.len() == 1 {
        return input[0].iter().fold(0, |acc, b| (acc << 1) ^ b);
    }

    let (zeroes, ones): (Vec<Vec<u32>>, Vec<Vec<u32>>) = input
        .into_iter()
        .map(|x| x.clone())
        .partition(|b| b[index] == 0);

    if decider(zeroes.len(), ones.len()) {
        filter(&zeroes, index + 1, decider)
    } else {
        filter(&ones, index + 1, decider)
    }
}

fn calculate_part1(input: &[Vec<u32>]) -> u32 {
    let counts = input.iter().fold(vec![0; 12], |acc, x| {
        acc.iter().zip(x.iter()).map(|(a, b)| a + b).collect()
    });

    let bits = counts
        .iter()
        .map(|n| ((*n as f32).ge(&(input.len() as f32 / 2.0))) as u32);
    let gamma = bits.clone().fold(0, |acc, b| (acc << 1) ^ b);
    let epsilon = bits.fold(0, |acc, b| (acc << 1) ^ (b == 0) as u32);

    gamma * epsilon
}

fn calculate_part2(input: &[Vec<u32>]) -> u32 {
    let oxygen_generator_rating = filter(&input, 0, |a, b| a > b);
    let co2_scrubber_rating = filter(&input, 0, |a, b| a <= b);

    oxygen_generator_rating * co2_scrubber_rating
}

fn main() {
    let input = get_input();
    let part1 = calculate_part1(&input);
    let part2 = calculate_part2(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
