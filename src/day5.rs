pub fn part1(input: &str) -> usize {
    let segments = read_input(input);
    let n = 1000;
    let mut count = 0;
    let grid = get_grid(n);
    let mut overlaps = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            if overlaps[i][j] >= 2 {continue}
            for segment in &segments {
                if !(segment.is_horizontal() || segment.is_vertical()) {
                    continue;
                }
                if segment.contains(grid[i][j]) {
                    overlaps[i][j] += 1
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if overlaps[i][j] >= 2 {
                count += 1
            }
        }
    }
    return count;
}

pub fn part2(input: &str) -> usize {
    0
}

fn get_grid(dim: usize) -> Vec<Vec<Point>> {
    let mut grid = Vec::with_capacity(dim);
    for i in 0..dim {
        let mut row = Vec::with_capacity(dim);
        for j in 0..dim {
            row.push(Point {
                x: j as isize,
                y: i as isize,
            });
        }
        grid.push(row);
    }
    return grid;
}

fn read_input(input: &str) -> Vec<Segment> {
    let map_segment = |l: &str| {
        let points: Vec<Point> = l.split(" -> ").map(|x| Point::from_str(x)).collect();
        Segment::new(points[0], points[1])
    };
    input.lines().map(|l| map_segment(l)).collect()
}

#[derive(Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn from_str(input: &str) -> Self {
        let v: Vec<isize> = input
            .split(",")
            .map(|c| c.parse::<isize>().unwrap())
            .collect();
        if let [x, y] = &v[..] {
            Point { x: *x, y: *y }
        } else {
            panic!()
        }
    }
}

struct Segment {
    start: Point,
    end: Point,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl Segment {
    fn new(p1: Point, p2: Point) -> Self {
        let min_x = isize::min(p1.x, p2.x);
        let max_x = isize::max(p1.x, p2.x);
        let min_y = isize::min(p1.y, p2.y);
        let max_y = isize::max(p1.y, p2.y);
        Segment {
            start: p1,
            end: p2,
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn contains(&self, other: Point) -> bool {
        if other.x < self.min_x || other.x > self.max_x {
            return false;
        }
        if other.y < self.min_y || other.y > self.max_y {
            return false;
        }
        if self.is_horizontal() {
            return other.y == self.start.y;
        } else if self.is_vertical() {
            return other.x == self.start.x;
        }
        false
    }
}

#[cfg(test)]
mod tests_day5 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";
        let result = part1(input);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part2() {
        let input = "";
        let result = part2(input);
        assert_eq!(result, 1924);
    }
}
