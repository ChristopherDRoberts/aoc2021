pub fn part1(input: &str) -> usize {
    let depths = read_input(input);
    lagged_compare(&depths, 1)
}

pub fn part2(input: &str) -> usize {
    let depths = read_input(input);
    lagged_compare(&depths, 3)
}

fn lagged_compare(depths: &Vec<usize>, lag: usize) -> usize {
    depths
        .into_iter()
        .skip(lag)
        .zip(depths.into_iter())
        .fold(0, |acc, zip| if zip.0 > zip.1 { acc + 1 } else { acc })
}

fn read_input(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse::<usize>().unwrap()).collect()
}

#[cfg(test)]
mod tests_day1 {
    use super::*;

    #[test]
    fn test_part1() {
        let depths = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
        let increases = part1(&depths);
        assert_eq!(increases, 7);
    }

    #[test]
    fn test_part2() {
        let depths = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
        let increases = part2(&depths);
        assert_eq!(increases, 5);
    }
}
