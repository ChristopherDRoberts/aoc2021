pub fn part1(input: &str) -> u32 {
    let (draws, mut boards) = read_input(input);
    for draw in draws {
        for board in &mut boards {
            board.mark(draw);
            if board.won {return board.score()}
        }
    }
    return 0;
}

pub fn part2(input: &str) -> u32 {
    let (draws, mut boards) = read_input(input);
    let mut winners_scores = Vec::new();
    for draw in draws {
        for board in &mut boards {
            if board.won {continue}
            board.mark(draw);
            if board.won {
                winners_scores.push(board.score());
            }
        }
    }
    return winners_scores.pop().unwrap();
}

fn read_input(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut draws = Vec::new();
    let mut boards = Vec::new();
    for (i, block) in input.split("\n\n").enumerate() {
        if i == 0 {
            draws = block
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            continue;
        }
        let mut board = Vec::new();
        for line in block.lines() {
            let clean_line = line.trim().replace("  ", " ");
            let row = clean_line
                .split(" ")
                .map(|c| c.parse::<u32>().unwrap())
                .collect();
            board.push(row);
        }
        boards.push(Board::new(board));
    }
    return (draws, boards);
}

struct Board {
    board: Vec<Vec<u32>>,
    marked: Vec<Vec<bool>>,
    dim: usize,
    won: bool,
    last_draw: u32,
}

impl Board {
    fn new(board: Vec<Vec<u32>>) -> Self {
        let n = board.len();
        Board {
            board: board,
            marked: vec![vec![false; n]; n],
            dim: n,
            won: false,
            last_draw: 0
        }
    }

    fn mark(&mut self, number: u32) {
        if self.won {return;}
        self.last_draw = number;
        for i in 0..self.dim {
            for j in 0..self.dim {
                if self.board[i][j] == number {
                    self.marked[i][j] = true
                };
            }
        }

        // check if winner
        // check rows
        for row in &self.marked {
            if row.into_iter().all(|x| *x == true) {
                self.won = true;
            }
        }

        // check cols
        for j in 0..self.dim {
            let mut wins = true;
            for i in 0..self.dim {
                wins = wins && self.marked[i][j];
            }
            if wins {
                self.won = true;
            }
        }
    }

    fn score(&self) -> u32 {
        if !self.won {return 0;}
        let mut unmarked_score = 0;
        for row in 0..self.dim {
            for col in 0..self.dim {
                if !self.marked[row][col] {
                    unmarked_score += self.board[row][col]
                };
            }
        }
        return unmarked_score * self.last_draw;
    }
}

#[cfg(test)]
mod tests_day4 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";
        let result = part1(input);
        assert_eq!(result, 4512);
    }

    #[test]
    fn test_part2() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";
        let result = part2(input);
        assert_eq!(result, 1924);
    }
}
