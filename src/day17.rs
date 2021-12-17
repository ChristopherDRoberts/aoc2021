pub fn part1(input: &str) -> isize {
    let target = read_input(input);
    let velocities = feasible_velocities(target);
    velocities.iter().map(|v| max_height(v)).max().unwrap()
}

pub fn part2(input: &str) -> usize {
    let target = read_input(input);
    let velocities = feasible_velocities(target);
    velocities.len()
}

fn read_input(input: &str) -> Target {
    // puzzle input has been manually pre-processed to xmin,xmax,ymin,ymax
    let parts: Vec<isize> = input
        .split(",")
        .map(|s| s.parse::<isize>().unwrap())
        .collect();
    Target {
        xmin: parts[0],
        xmax: parts[1],
        ymin: parts[2],
        ymax: parts[3],
    }
}

fn position(velocity: &Vec2, steps: usize) -> Vec2 {
    let n = steps as isize;

    // Don't need to consider negative initial x velocity case since our
    // target is to the right of us
    let x = if n >= velocity.x {
        (velocity.x * (velocity.x + 1)) / 2
    } else {
        n * velocity.x - n * (n - 1) / 2
    };

    let y = n * velocity.y - n * (n - 1) / 2;
    Vec2 { x, y }
}

fn max_height(velocity: &Vec2) -> isize {
    velocity.y * (velocity.y + 1) / 2
}

fn feasible_velocities(target: Target) -> Vec<Vec2> {
    let mut velocities = Vec::new();
    let min_vx = ((-1.0 + ((1 + 8 * target.xmin) as f64).sqrt()) / 2.0).floor() as isize;
    for vx in min_vx..=target.xmax {
        for vy in target.ymin..=target.ymin.abs() {
            let velocity = Vec2 { x: vx, y: vy };
            for n in 1.. {
                let pos = position(&velocity, n);
                if target.hit(&pos) {
                    velocities.push(velocity);
                    break;
                };
                if target.passed(&pos) {
                    break;
                };
            }
        }
    }
    return velocities;
}

struct Target {
    xmin: isize,
    xmax: isize,
    ymin: isize,
    ymax: isize,
}

impl Target {
    fn hit(&self, point: &Vec2) -> bool {
        point.x >= self.xmin && point.x <= self.xmax && point.y >= self.ymin && point.y <= self.ymax
    }

    fn passed(&self, point: &Vec2) -> bool {
        point.x > self.xmax || point.y < self.ymin
    }
}

#[derive(Debug)]
struct Vec2 {
    x: isize,
    y: isize,
}

#[cfg(test)]
mod tests_day17 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "20,30,-10,-5";
        let result = part1(input);
        assert_eq!(result, 45);
    }

    #[test]
    fn test_part2() {
        let input = "20,30,-10,-5";
        let result = part2(input);
        assert_eq!(result, 112);
    }
}
