use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn part1(input: &str) -> isize {
    let grid = read_input(input);
    let mut low_points = HashMap::new();
    for (coords, point) in &grid {
        let adjacents = adjacent_points(*coords, &grid);
        if is_low_point(*point, &adjacents) {
            low_points.insert(*coords, *point);
        }
    }
    low_points.values().fold(0, |acc, x| acc + x + 1)
}

pub fn part2(input: &str) -> usize {
    let grid = read_input(input);
    let mut low_points = HashMap::new();
    for (coords, point) in &grid {
        let adjacents = adjacent_points(*coords, &grid);
        if is_low_point(*point, &adjacents) {
            low_points.insert(*coords, *point);
        }
    }
    let new_grid = transform_grid(grid);
    let mut basin_sizes = BinaryHeap::new();
    for point in low_points.keys() {
        basin_sizes.push(basin_size(*point, &new_grid));
    }
    basin_sizes.iter().take(3).product()
}

fn basin_size(point: (isize, isize), grid: &HashMap<(isize, isize), isize>) -> usize {
    let mut size = 1;
    let mut visited_points = HashSet::new();
    visited_points.insert(point);
    let mut unvisted_points = VecDeque::new();
    let adjacents = adjacent_points(point, grid);
    for (adj, v) in adjacents {
        if !visited_points.contains(&adj) && v != 0 {
            unvisted_points.push_back(adj);
        }
    }
    while unvisted_points.len() > 0 {
        let test_point = unvisted_points.pop_front().unwrap();
        visited_points.insert(test_point);
        if grid.get(&test_point).unwrap() == &1 {
            size += 1
        }
        let adjs = adjacent_points(test_point, grid);
        for (adj, v) in adjs {
            if !visited_points.contains(&adj) && v != 0 {
                unvisted_points.push_back(adj)
            }
        }
    }
    let x = 1;
    return size;
}

fn read_input(input: &str) -> HashMap<(isize, isize), isize> {
    let mut grid = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.trim().chars().enumerate() {
            grid.insert((i as isize, j as isize), c.to_digit(10).unwrap() as isize);
        }
    }
    let x = 1;
    return grid;
}

fn transform_grid(grid: HashMap<(isize, isize), isize>) -> HashMap<(isize, isize), isize> {
    let mut new_grid = HashMap::new();
    for (k, v) in grid {
        let x = if v < 9 { 1 } else { 0 };
        new_grid.insert(k, x);
    }
    return new_grid;
}

fn adjacent_points(
    point: (isize, isize),
    grid: &HashMap<(isize, isize), isize>,
) -> HashMap<(isize, isize), isize> {
    let mut points = HashMap::new();
    let test_points = vec![
        (point.0 - 1, point.1),
        (point.0 + 1, point.1),
        (point.0, point.1 - 1),
        (point.0, point.1 + 1),
    ];
    for tpoint in test_points {
        if let Some(v) = grid.get(&tpoint) {
            points.insert(tpoint, *v);
        }
    }
    return points;
}

fn is_low_point(point: isize, adjacents: &HashMap<(isize, isize), isize>) -> bool {
    adjacents.values().all(|x| point < *x)
}

#[cfg(test)]
mod tests_day8 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"2199943210
        3987894921
        9856789892
        8767896789
        9899965678
        ";
        let result = part1(input);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_part2() {
        let input = "2199943210
        3987894921
        9856789892
        8767896789
        9899965678
        ";
        let result = part2(input);
        assert_eq!(result, 1134);
    }
}
