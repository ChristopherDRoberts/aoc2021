pub fn part1(input: &str) -> usize {
    let mut grid = read_input(input);
    let mut total_flashes = 0;
    for _ in 0..100 {
        total_flashes += evolve(&mut grid);
    }
    return total_flashes;
}

pub fn part2(input: &str) -> usize {
    let mut grid = read_input(input);
    let mut step = 0;
    loop {
        evolve(&mut grid);
        step += 1;
        if synched(&grid) {
            break;
        };
    }
    return step;
}

fn read_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.trim().chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn evolve(grid: &mut Vec<Vec<u32>>) -> usize {
    // increment
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            grid[i][j] += 1;
        }
    }

    // loop until all flashes done
    let mut flash_count = 0;
    let mut flashed = vec![vec![false; grid[0].len()]; grid.len()];
    loop {
        let mut any_flashed = false;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] > 9 && !flashed[i][j] {
                    flash_count += 1;
                    flashed[i][j] = true;
                    let neighbours = neighbours(i, j, grid);
                    for n in neighbours {
                        let (n_i, n_j) = n;
                        grid[n_i][n_j] += 1;
                    }
                    any_flashed = true;
                }
            }
        }
        if !any_flashed {
            break;
        }
    }

    // zero flashed indices
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if flashed[i][j] {
                grid[i][j] = 0;
            }
        }
    }

    return flash_count;
}

fn neighbours(row: usize, col: usize, grid: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;
    for dy in vec![-1, 0, 1] {
        for dx in vec![-1, 0, 1] {
            if dx == 0 && dy == 0 {
                continue;
            }
            let tmpy = row as isize + dy;
            let tmpx = col as isize + dx;
            if (tmpy >= 0 && tmpy < height) && (tmpx >= 0 && tmpx < width) {
                neighbours.push((tmpy as usize, tmpx as usize));
            }
        }
    }
    return neighbours;
}

fn synched(grid: &Vec<Vec<u32>>) -> bool {
    for row in grid {
        for x in row {
            if *x != 0 {
                return false;
            }
        }
    }
    return true;
}

#[cfg(test)]
mod tests_day11 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526";
        let result = part1(input);
        assert_eq!(result, 1656);
    }

    #[test]
    fn test_part2() {
        let input = "5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526";
        let result = part2(input);
        assert_eq!(result, 195);
    }
}
