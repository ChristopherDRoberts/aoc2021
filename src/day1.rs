pub fn part1(input: &String) -> usize {
    let parsed_input = read_input(input);
    part1_impl(&parsed_input)
}

pub fn part2(input: &String) -> usize {
    let parsed_input = read_input(input);
    part2_impl(&parsed_input)
}

fn part1_impl(depths: &Vec<usize>) -> usize {
    lagged_compare(depths, 1)
}

fn part2_impl(depths: &Vec<usize>) -> usize {
    lagged_compare(depths, 3)
}

fn lagged_compare(depths: &Vec<usize>, lag: usize) -> usize {
    depths
        .into_iter()
        .skip(lag)
        .zip(depths.into_iter())
        .fold(0, |acc, zip| if zip.0 > zip.1 { acc + 1 } else { acc })
}

fn read_input(input: &String) -> Vec<usize> {
    let mut parsed_input = Vec::new();
    for line in input.lines() {
        parsed_input.push(line.parse::<usize>().unwrap());
    }
    return parsed_input;
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn test_part1() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let increases = part1_impl(&depths);
        assert_eq!(increases, 7);
    }

    #[test]
    fn test_part2() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let increases = part2_impl(&depths);
        assert_eq!(increases, 5);
    }
}
