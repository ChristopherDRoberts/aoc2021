pub fn part1(input: &str) -> usize {
    get_population(input, 80)
}

pub fn part2(input: &str) -> usize {
    get_population(input, 256)
}

fn get_population(input: &str, days: usize) -> usize {
    let mut state = read_input(input);
    for _i in 0..days {
        advance(&mut state);
    }
    return state.into_iter().sum();
}

fn read_input(input: &str) -> Vec<usize> {
    let mut counts = vec![0; 9];
    let timers = input.split(",").map(|c| c.parse::<usize>().unwrap());
    for timer in timers {
        counts[timer] += 1;
    }
    return counts;
}

fn advance(state: &mut Vec<usize>) {
    let new_fish = state[0];
    let n = state.len();
    for i in 0..state.len() - 1 {
        state[i] = state[i + 1];
    }
    state[n - 1] = new_fish;
    state[6] += new_fish;
}

#[cfg(test)]
mod tests_day6 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "3,4,3,1,2";
        let result = part1(input);
        assert_eq!(result, 5934);
    }

    #[test]
    fn test_part2() {
        let input = "3,4,3,1,2";
        let result = part2(input);
        assert_eq!(result, 26984457539);
    }
}
