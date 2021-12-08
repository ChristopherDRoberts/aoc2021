pub fn part1(input: &str) -> isize {
    let coordinates = read_input(input);
    let min_position = coordinates[(coordinates.len() / 2)];
    let mut fuel = 0;
    for coordinate in coordinates {
        fuel += (coordinate - min_position).abs();
    }
    return fuel;
}

pub fn part2(input: &str) -> isize {
    let coordinates = read_input(input);
    let n = coordinates.len();
    let sum: isize = coordinates.iter().sum();
    let min_position = ((sum as f64) / (n as f64)).floor() as isize;
    let min_grid = vec![min_position, min_position + 1];
    let mut min_fuel = isize::MAX;
    for pos in min_grid {
        let mut fuel = 0;
        for coordinate in &coordinates {
            let n = (coordinate - pos).abs();
            fuel += n * (n + 1) / 2;
        }
        if fuel < min_fuel {
            min_fuel = fuel
        };
    }
    return min_fuel;
}

fn read_input(input: &str) -> Vec<isize> {
    let mut coordinates: Vec<isize> = input
        .split(",")
        .map(|c| c.parse::<isize>().unwrap())
        .collect();
    coordinates.sort();
    return coordinates;
}

#[cfg(test)]
mod tests_day7 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let result = part1(input);
        assert_eq!(result, 37);
    }

    #[test]
    fn test_part2() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let result = part2(input);
        assert_eq!(result, 168);
    }
}
