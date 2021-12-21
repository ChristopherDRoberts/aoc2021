use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let board: Vec<usize> = (1..=10).collect();
    let mut scores = vec![0; 2];
    let mut pos: Vec<usize> = read_input(input).iter().map(|p| p - 1).collect();
    let mut rounds = 0;
    loop {
        pos[0] = (pos[0] + (6 + 9 * rounds)) % 10;
        rounds += 1;
        scores[0] += board[pos[0]];
        if scores[0] >= 1000 {
            break;
        }

        pos[1] = (pos[1] + (6 + 9 * rounds)) % 10;
        rounds += 1;
        scores[1] += board[pos[1]];
        if scores[1] >= 1000 {
            break;
        }
    }
    scores.iter().min().unwrap() * (3 * rounds)
}

pub fn part2(input: &str) -> usize {
    let board: Vec<usize> = (1..=10).collect();
    let pos: Vec<usize> = read_input(input).iter().map(|p| p - 1).collect();
    let positions = (pos[0], pos[1]);
    let mut cache = HashMap::new();
    let wins = dirac_dice((0,0), positions, &board, true, &mut cache);
    if wins.0 > wins.1 {wins.0} else {wins.1}
}

fn dirac_dice(
    scores: (usize, usize),
    positions: (usize, usize),
    board: &Vec<usize>,
    p1_to_play: bool,
    cache: &mut HashMap<(usize, usize, usize, usize, bool), (usize, usize)>,
) -> (usize, usize) {
    let key = (scores.0, scores.1, positions.0, positions.1, p1_to_play);
    if cache.contains_key(&key) {
        return cache[&key];
    }
    let mut wins = (0, 0);
    if p1_to_play {
        if scores.1 >= 21 {
            return (0, 1);
        };
        for i in 1..=3 {
            for j in 1..=3 {
                for k in 1..=3 {
                    let p1_position = (positions.0 + i + j + k) % 10;
                    let p1_score = scores.0 + board[p1_position];
                    let (p1_wins, p2_wins) = dirac_dice(
                        (p1_score, scores.1),
                        (p1_position, positions.1),
                        board,
                        !p1_to_play,
                        cache,
                    );
                    wins.0 += p1_wins;
                    wins.1 += p2_wins;
                }
            }
        }
        cache.insert(key, wins);
    }
    if !p1_to_play {
        if scores.0 >= 21 {
            return (1, 0);
        };
        for i in 1..=3 {
            for j in 1..=3 {
                for k in 1..=3 {
                    let p2_position = (positions.1 + i + j + k) % 10;
                    let p2_score = scores.1 + board[p2_position];
                    let (p1_wins, p2_wins) = dirac_dice(
                        (scores.0, p2_score),
                        (positions.0, p2_position),
                        board,
                        !p1_to_play,
                        cache,
                    );
                    wins.0 += p1_wins;
                    wins.1 += p2_wins;
                }
            }
        }
        cache.insert(key, wins);
    }
    return wins;
}

fn read_input(input: &str) -> Vec<usize> {
    // manually pre-processed input to "p1 start, p2 start"
    input
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests_day19 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "4,8";
        let result = part1(input);
        assert_eq!(result, 739785);
    }

    #[test]
    fn test_part2() {
        let input = "4,8";
        let result = part2(input);
        assert_eq!(result, 444356092776315);
    }
}
