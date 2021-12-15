use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn part1(input: &str) -> usize {
    let grid = read_input(input);
    let (vertices, adjacencies) = get_vertices_and_adjacencies(&grid);
    return shortest_path(0, vertices.len() - 1, &vertices, &adjacencies).unwrap();
}

pub fn part2(input: &str) -> usize {
    let grid = read_input(input);
    let expanded_grid = expand_grid(&grid, 5);
    let (vertices, adjacencies) = get_vertices_and_adjacencies(&expanded_grid);
    return shortest_path(0, vertices.len() - 1, &vertices, &adjacencies).unwrap();
}

fn read_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn expand_grid(grid: &Vec<Vec<usize>>, multiplier: usize) -> Vec<Vec<usize>> {
    let n = grid.len();
    let m = n * multiplier;
    let mut expanded_grid = vec![vec![0; m]; m];
    for i in 0..n{
        for j in 0..n{
            expanded_grid[i][j] = grid[i][j];
        }
    }
    for i in n..m{
        for j in 0..n{
            let x = expanded_grid[(i-n) as usize][j] + 1;
            expanded_grid[i][j] = if x == 10 {1} else {x};
        }
    }
    for i in 0..m{
        for j in n..m {
            let x = expanded_grid[i][(j-n) as usize] + 1;
            expanded_grid[i][j] = if x == 10 {1} else {x};
        }
    }
    return expanded_grid;
}

fn get_vertices_and_adjacencies(grid: &Vec<Vec<usize>>) -> (Vec<Vertex>, Vec<Vec<Vertex>>) {
    let mut vertices = Vec::new();
    let n = grid.len();
    for i in 0..n {
        for j in 0..n {
            vertices.push(Vertex {
                index: i * n + j,
                weight: grid[i][j],
            })
        }
    }
    let mut adj = Vec::new();
    for i in 0..n {
        for j in 0..n {
            let adjs = get_adjacent_vertices(i, j, n, &vertices);
            adj.push(adjs);
        }
    }
    return (vertices, adj);
}

fn shortest_path(
    start: usize,
    end: usize,
    vertices: &Vec<Vertex>,
    adjacencies: &Vec<Vec<Vertex>>,
) -> Option<usize> {
    let mut distances = vec![usize::MAX; vertices.len()];
    distances[start] = 0;
    let mut unvisited = BinaryHeap::new();
    unvisited.push(NextVertex {
        index: start,
        distance: distances[start],
    });
    while let Some(NextVertex { index, distance }) = unvisited.pop() {
        if index == end {
            return Some(distance);
        };
        if distance > distances[index] {
            continue;
        }
        for adj in &adjacencies[index] {
            let next = NextVertex {
                index: adj.index,
                distance: distance + adj.weight,
            };
            if next.distance < distances[next.index] {
                unvisited.push(next);
                distances[next.index] = next.distance;
            }
        }
    }
    return None;
}

#[derive(Copy, Clone)]
struct Vertex {
    index: usize,
    weight: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct NextVertex {
    index: usize,
    distance: usize,
}

impl Ord for NextVertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.index.cmp(&other.index))
    }
}

impl PartialOrd for NextVertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_adjacent_vertices(
    row: usize,
    col: usize,
    grid_size: usize,
    vertices: &Vec<Vertex>,
) -> Vec<Vertex> {
    let mut adj = Vec::new();
    for r in [row as isize - 1, row as isize + 1] {
        if r >= 0 && r < grid_size as isize {
            let index = (r as usize) * grid_size + col;
            adj.push(Vertex {
                index,
                weight: vertices[index].weight,
            });
        }
    }

    for c in [col as isize - 1, col as isize + 1] {
        if c >= 0 && c < grid_size as isize {
            let index = row * grid_size + (c as usize);
            adj.push(Vertex {
                index,
                weight: vertices[index].weight,
            });
        }
    }
    return adj;
}

#[cfg(test)]
mod tests_day15 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581";
        let result = part1(input);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part2() {
        let input = "1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581";
        let result = part2(input);
        assert_eq!(result, 315);
    }
}
