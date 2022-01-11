pub fn part1(input: &str) -> usize {
    let mut steps = 0;
    let mut grid = read_input(input);
    loop {
        let new_grid = evolve(&grid);
        steps += 1;
        if stopped(&grid, &new_grid) {
            break;
        }
        grid = new_grid;
    }
    return steps;
}

pub fn part2(input: &str) -> usize {
    0
}

fn stopped(prev_grid: &Vec<Vec<char>>, next_grid: &Vec<Vec<char>>) -> bool {
    let stopped = true;
    for i in 0..prev_grid.len() {
        for j in 0..prev_grid[0].len() {
            if prev_grid[i][j] != next_grid[i][j] {
                return false;
            }
        }
    }
    return stopped;
}

fn evolve(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let tmp = move_east(grid);
    let new_grid = move_south(&tmp);
    return new_grid;
}

fn move_east(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut new_grid = grid.clone();
    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == '>' && grid[i][(j + 1) % width] == '.' {
                new_grid[i][j] = '.';
                new_grid[i][(j + 1) % width] = '>';
            }
        }
    }
    return new_grid;
}

fn move_south(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut new_grid = grid.clone();
    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == 'v' && grid[(i + 1) % height][j] == '.' {
                new_grid[i][j] = '.';
                new_grid[(i + 1) % height][j] = 'v';
            }
        }
    }
    return new_grid;
}

fn read_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.trim().chars().collect()).collect()
}

#[cfg(test)]
mod tests_day25 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "v...>>.vv>
        .vv>>.vv..
        >>.>v>...v
        >>v>>.>.v.
        v>v.vv.v..
        >.>>..v...
        .vv..>.>v.
        v.v..>>v.v
        ....v..v.>";
        let result = part1(input);
        assert_eq!(result, 58);
    }

    #[test]
    fn test_part2() {
        let input = "";
        let result = part2(input);
        assert_eq!(result, 112);
    }
}
