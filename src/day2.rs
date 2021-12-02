use Direction::*;

pub fn part1(input: &str) -> isize {
    let directions = read_input(input);
    let mut position = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };
    for direction in directions {
        position.mov(direction);
    }
    position.horizontal * position.depth
}

pub fn part2(input: &str) -> isize {
    let directions = read_input(input);
    let mut position = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };
    for direction in directions {
        position.mov_with_aim(direction);
    }
    position.horizontal * position.depth
}

fn read_input(input: &str) -> Vec<Direction> {
    let map_direction = |direction_pair: Vec<&str>| match direction_pair[0] {
        "forward" => Direction::Forward(direction_pair[1].parse::<isize>().unwrap()),
        "up" => Direction::Up(direction_pair[1].parse::<isize>().unwrap()),
        "down" => Direction::Down(direction_pair[1].parse::<isize>().unwrap()),
        _ => panic!(),
    };
    let direction_pairs: Vec<Vec<&str>> = input.lines().map(|l| l.split(" ").collect()).collect();
    let directions = direction_pairs.into_iter().map(map_direction).collect();
    return directions;
}

#[derive(Debug)]
enum Direction {
    Forward(isize),
    Up(isize),
    Down(isize),
}

#[derive(Debug)]
struct Position {
    horizontal: isize,
    depth: isize,
    aim: isize,
}

impl Position {
    fn mov(&mut self, direction: Direction) {
        match direction {
            Forward(x) => self.horizontal += x,
            Up(x) => self.depth -= x,
            Down(x) => self.depth += x,
        }
    }

    fn mov_with_aim(&mut self, direction: Direction) {
        match direction {
            Forward(x) => {
                self.horizontal += x;
                self.depth += self.aim * x
            }
            Up(x) => self.aim -= x,
            Down(x) => self.aim += x,
        }
    }
}

#[cfg(test)]
mod test_day2 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        let result = part1(input);
        assert_eq!(result, 150);
    }

    #[test]
    fn test_part2() {
        let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        let result = part2(input);
        assert_eq!(result, 900);
    }
}
