pub enum Direction {
    Forward,
    Up,
    Down
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<(Direction, usize)> {
    input
        .lines()
        .map(|l| l.split(" ").collect::<Vec<_>>())
        .map(|pair| (parse_direction(pair[0]), pair[1].parse().unwrap()))
        .collect()
}

fn parse_direction(dir: &str) -> Direction {
    match dir {
        "forward" => Direction::Forward,
        "up" => Direction::Up,
        "down" => Direction::Down,
        _ => panic!()
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &[(Direction, usize)]) -> usize {
    let (x, y) = input.iter()
        .fold((0, 0), |(x, y), (dir, amt)| match dir {
            Direction::Forward => (x + amt, y),
            Direction::Up => (x, y - amt),
            Direction::Down => (x, y + amt),
        });
    x * y
}

#[aoc(day2, part2)]
pub fn part2(input: &[(Direction, usize)]) -> usize {
    let (x, y, _aim) = input.iter()
        .fold((0, 0, 0), |(x, y, aim), (dir, amt)| match dir {
            Direction::Forward => (x + amt, y + aim * amt, aim),
            Direction::Up => (x, y, aim - amt),
            Direction::Down => (x, y, aim + amt),
        });
    x * y
}
