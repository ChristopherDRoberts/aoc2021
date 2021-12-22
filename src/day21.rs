use regex::Regex;
use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let mut scores = vec![0; 2];
    let mut pos: Vec<usize> = read_input(input).iter().map(|p| p - 1).collect();
    let mut rounds = 0;
    loop {
        pos[0] = (pos[0] + (6 + 9 * rounds)) % 10;
        rounds += 1;
        scores[0] += pos[0] + 1;
        if scores[0] >= 1000 {
            break;
        }

        pos[1] = (pos[1] + (6 + 9 * rounds)) % 10;
        rounds += 1;
        scores[1] += pos[1] + 1;
        if scores[1] >= 1000 {
            break;
        }
    }
    scores.iter().min().unwrap() * (3 * rounds)
}

pub fn part2(input: &str) -> usize {
    let pos: Vec<usize> = read_input(input).iter().map(|p| p - 1).collect();
    let positions = (pos[0], pos[1]);
    let mut cache = HashMap::new();
    let wins = dirac_dice((0, 0), positions, true, &mut cache);
    if wins.0 > wins.1 {
        wins.0
    } else {
        wins.1
    }
}

fn dirac_dice(
    scores: (usize, usize),
    positions: (usize, usize),
    p1_to_play: bool,
    cache: &mut HashMap<(usize, usize, usize, usize, bool), (usize, usize)>,
) -> (usize, usize) {
    let key = (scores.0, scores.1, positions.0, positions.1, p1_to_play);
    if cache.contains_key(&key) {
        return cache[&key];
    }
    let mut wins = (0, 0);
    let mult = [1, 3, 6, 7, 6, 3, 1];
    let steps = [3, 4, 5, 6, 7, 8, 9]; // reduce down to permutations with the same sum
    if p1_to_play {
        if scores.1 >= 21 {
            return (0, 1);
        };
        for i in 0..steps.len() {
            let p1_position = (positions.0 + steps[i]) % 10;
            let p1_score = scores.0 + p1_position + 1;
            let (p1_wins, p2_wins) = dirac_dice(
                (p1_score, scores.1),
                (p1_position, positions.1),
                !p1_to_play,
                cache,
            );
            wins.0 += p1_wins * mult[i];
            wins.1 += p2_wins * mult[i];
        }
        cache.insert(key, wins);
    }
    if !p1_to_play {
        if scores.0 >= 21 {
            return (1, 0);
        };
        for i in 0..steps.len() {
            let p2_position = (positions.1 + steps[i]) % 10;
            let p2_score = scores.1 + p2_position + 1;
            let (p1_wins, p2_wins) = dirac_dice(
                (scores.0, p2_score),
                (positions.0, p2_position),
                !p1_to_play,
                cache,
            );
            wins.0 += p1_wins * mult[i];
            wins.1 += p2_wins * mult[i];
        }
        cache.insert(key, wins);
    }
    return wins;
}

fn read_input(input: &str) -> Vec<usize> {
    let re = Regex::new(r"Player \d+ starting position: (\d+)").unwrap();
    let mut pos = Vec::with_capacity(2);
    for (i, line) in input.lines().enumerate() {
        let captures = re.captures(line.trim()).unwrap();
        pos.push((&captures[1]).parse::<usize>().unwrap());
    }
    return pos;
}

#[cfg(test)]
mod tests_day21 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Player 1 starting position: 4
        Player 2 starting position: 8";
        let result = part1(input);
        assert_eq!(result, 739785);
    }

    #[test]
    fn test_part2() {
        let input = "Player 1 starting position: 4
        Player 2 starting position: 8";
        let result = part2(input);
        assert_eq!(result, 444356092776315);
    }
}
