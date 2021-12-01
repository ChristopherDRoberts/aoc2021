pub fn part1(input: &String) -> usize {
    let parsed_input = read_input(input);
    part1_impl(&parsed_input)
}

pub fn part2(input: &String) -> usize {
    let parsed_input = read_input(input);
    part2_impl(&parsed_input)
}

fn part1_impl(depths: &Vec<usize>) -> usize {
    depths
        .into_iter()
        .skip(1)
        .zip(depths.into_iter())
        .fold(0, |acc, zip| if zip.0 > zip.1 { acc + 1 } else { acc })
}

fn part2_impl(depths: &Vec<usize>) -> usize {
    let mut windows = depths.windows(3);
    let mut prev = window_sum(windows.next().unwrap());
    let mut sum = 0;
    for window in windows {
        let next = window_sum(window);
        if next > prev {
            sum += 1;
        }
        prev = next;
    }
    return sum;
}

fn read_input(input: &String) -> Vec<usize> {
    let mut parsed_input = Vec::new();
    for line in input.lines() {
        parsed_input.push(line.parse::<usize>().unwrap());
    }
    return parsed_input;
}

fn window_sum(window: &[usize]) -> usize{
    let mut sum = 0;
    for elem in window {
        sum += elem;
    }
    return sum;
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
