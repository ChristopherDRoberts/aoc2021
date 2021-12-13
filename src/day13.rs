use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    let (points, folds) = read_input(input);
    let mut new_points = HashSet::new();
    for point in points {
        if !point.can_fold(&folds[0]) {
            new_points.insert(point);
            continue;
        }
        let folded_point = point.fold(&folds[0]);
        new_points.insert(folded_point);
    }
    return new_points.len();
}

pub fn part2(input: &str) -> usize {
    let (points, folds) = read_input(input);
    let mut prev = points.clone();
    for fold in folds {
        let mut temp = HashSet::new();
        for point in &prev {
            if !point.can_fold(&fold) {
                temp.insert(*point);
                continue;
            }
            let folded_point = point.fold(&fold);
            temp.insert(folded_point);
        }
        prev = temp;
    }
    let grid = grid(prev);
    let mut signature = String::new(); // hopefully to catch errors if ever this is rewritten
    for row in grid {
        signature.push_str(&row.iter().filter(|c| **c == '#').count().to_string());
        println!("{}", row.iter().collect::<String>());
    }
    return signature.parse().unwrap();
}

fn read_input(input: &str) -> (HashSet<Point>, Vec<Fold>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let mut points = HashSet::new();
    for p in parts[0].lines() {
        let coords: Vec<isize> = p
            .trim()
            .split(",")
            .map(|x| x.parse::<isize>().unwrap())
            .collect();
        points.insert(Point::new(coords[0], coords[1]));
    }
    let mut folds = Vec::new();
    for i in parts[1].lines() {
        let i_parts: Vec<&str> = i.trim().split("=").collect();
        let axis = i_parts[0].chars().last().unwrap();
        let value = i_parts[1].parse::<isize>().unwrap();
        match axis {
            'x' => folds.push(Fold::X(value)),
            'y' => folds.push(Fold::Y(value)),
            _ => (),
        }
    }
    return (points, folds);
}

fn grid(points: HashSet<Point>) -> Vec<Vec<char>> {
    let mut max_x = 0;
    let mut max_y = 0;
    for p in &points {
        if p.x > max_x {
            max_x = p.x
        }
        if p.y > max_y {
            max_y = p.y
        }
    }
    let mut grid = vec![vec![' '; max_x as usize + 1]; max_y as usize + 1];
    for p in &points {
        let row = p.y as usize;
        let col = p.x as usize;
        grid[row][col] = '#';
    }
    return grid;
}

enum Fold {
    X(isize),
    Y(isize),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    fn fold(&self, fold: &Fold) -> Point {
        match fold {
            Fold::X(value) => Point {
                x: 2 * value - self.x,
                y: self.y,
            },
            Fold::Y(value) => Point {
                x: self.x,
                y: 2 * value - self.y,
            },
        }
    }

    fn can_fold(&self, fold: &Fold) -> bool {
        match fold {
            Fold::X(value) => *value < self.x,
            Fold::Y(value) => *value < self.y,
        }
    }
}

#[cfg(test)]
mod tests_day13 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        let result = part1(input);
        assert_eq!(result, 17);
    }

    #[test]
    fn test_part2() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        let result = part2(input);
        assert_eq!(result, 103);
    }
}
