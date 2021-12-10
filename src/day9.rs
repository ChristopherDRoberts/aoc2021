use std::collections::BinaryHeap;
use std::collections::VecDeque;

pub fn part1(input: &str) -> u32 {
    let map = read_input(input);
    let minima = map.minima();
    let mut risk = 0;
    for minimum in minima {
        risk += minimum.value + 1;
    }
    return risk;
}

pub fn part2(input: &str) -> u32 {
    let map = read_input(input);
    let minima = map.minima();
    let mut basins_sizes = BinaryHeap::new();
    for m in minima {
        let size = map.basin_size(m);
        basins_sizes.push(size);
    }
    basins_sizes.iter().take(3).product()
}

fn read_input(input: &str) -> HeightMap {
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.trim().chars() {
            row.push(c.to_digit(10).unwrap());
        }
        grid.push(row);
    }
    return HeightMap::new(grid);
}

struct Vertex {
    index: usize,
    value: u32,
}

struct HeightMap {
    adjacency: Vec<Vec<Vertex>>,
    vertices: Vec<Vertex>,
}

impl HeightMap {
    fn new(grid: Vec<Vec<u32>>) -> Self {
        let mut index = 0;
        let mut vertices = Vec::new();
        let mut adjacency = Vec::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                vertices.push(Vertex {
                    index,
                    value: grid[i][j],
                });
                let adjacents = HeightMap::populated_adjacents(i, j, &grid);
                adjacency.push(adjacents);
                index += 1;
            }
        }
        return HeightMap {
            adjacency,
            vertices,
        };
    }

    fn populated_adjacents(row: usize, col: usize, grid: &Vec<Vec<u32>>) -> Vec<Vertex> {
        let mut adjacents = Vec::new();
        let height = grid.len();
        let width = grid[0].len();

        if (row as isize - 1) >= 0 {
            adjacents.push(Vertex {
                index: (row - 1) * width + col,
                value: grid[row - 1][col],
            })
        }

        if (row + 1) < height {
            adjacents.push(Vertex {
                index: (row + 1) * width + col,
                value: grid[row + 1][col],
            })
        }

        if (col as isize - 1) >= 0 {
            adjacents.push(Vertex {
                index: row * width + (col - 1),
                value: grid[row][col - 1],
            })
        }

        if (col + 1) < width {
            adjacents.push(Vertex {
                index: row * width + (col + 1),
                value: grid[row][col + 1],
            })
        }

        return adjacents;
    }

    fn minima(&self) -> Vec<&Vertex> {
        let mut minima = Vec::new();
        for i in 0..self.adjacency.len() {
            let test_vertex = &self.vertices[i];
            if self.adjacency[i].iter().all(|vertex| vertex.value > test_vertex.value){
                minima.push(test_vertex);
            }
        }
        return minima;
    }

    fn basin_size(&self, vertex: &Vertex) -> u32 {
        let mut visited = vec![false; self.vertices.len()];
        let mut unvisited = VecDeque::new();
        let mut size = 1;
        visited[vertex.index] = true;
        for v in &self.adjacency[vertex.index]{
            if v.value != 9 {
                unvisited.push_back(v);
            }
        }
        while unvisited.len() > 0 {
            let tv = unvisited.pop_front().unwrap();
            if visited[tv.index] {continue}
            size += 1;
            visited[tv.index] = true;
            for v in &self.adjacency[tv.index]{
                if v.value != 9 && !visited[v.index] {
                    unvisited.push_back(v);
                }
            }
        }
        return size;
    }
}

#[cfg(test)]
mod tests_day9 {
    use super::*;

    // #[test]
    // fn test_read_input() {
    //     let input = "012
    //     345
    //     678";
    //     let graph = read_input(input);
    //     let x = 1;
    // }

    #[test]
    fn test_part1() {
        let input = "2199943210
        3987894921
        9856789892
        8767896789
        9899965678";
        let result = part1(input);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_part2() {
        let input = "2199943210
        3987894921
        9856789892
        8767896789
        9899965678";
        let result = part2(input);
        assert_eq!(result, 1134);
    }
}
