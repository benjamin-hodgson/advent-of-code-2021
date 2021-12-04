#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[usize]) -> usize {
    count_increases(input)
}
#[aoc(day1, part2)]
pub fn part2(input: &[usize]) -> usize {
    count_increases(&sum_threes(input))
}

fn count_increases(input: &[usize]) -> usize {
    input.iter()
        .skip(1)
        .zip(input.iter())
        .filter(|(cur, prev)| cur > prev)
        .count()
}

fn sum_threes(input: &[usize]) -> Vec<usize> {
    input.iter().skip(2)
        .zip(input.iter().skip(1))
        .zip(input.iter())
        .map(|((x, y), z)| x + y + z)
        .collect()
}
