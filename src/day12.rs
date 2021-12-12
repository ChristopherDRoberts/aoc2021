use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let map = read_input(input);
    let start = Cave {
        label: "start",
        small: true,
    };
    let end = Cave {
        label: "end",
        small: true,
    };
    let visited_caves = Vec::<Cave>::new();
    let paths = generate_paths_part1(start, end, visited_caves, &map);
    return paths.len();
}

pub fn part2(input: &str) -> usize {
    let map = read_input(input);
    let start = Cave {
        label: "start",
        small: true,
    };
    let end = Cave {
        label: "end",
        small: true,
    };
    let visited_caves = Vec::<Cave>::new();
    let paths = generate_paths_part2(start, end, visited_caves, &map);
    return paths.len();
}

fn read_input(input: &str) -> HashMap<Cave, Vec<Cave>> {
    let mut map = HashMap::<Cave, Vec<Cave>>::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.trim().split("-").collect();
        let start: Cave;
        if parts[0].to_lowercase() == parts[0] {
            start = Cave {
                label: parts[0],
                small: true,
            };
        } else {
            start = Cave {
                label: parts[0],
                small: false,
            };
        }
        let end: Cave;
        if parts[1].to_lowercase() == parts[1] {
            end = Cave {
                label: parts[1],
                small: true,
            };
        } else {
            end = Cave {
                label: parts[1],
                small: false,
            };
        }

        (*map.entry(start).or_insert(Vec::new())).push(end);
        (*map.entry(end).or_insert(Vec::new())).push(start);
    }
    return map;
}

fn generate_paths_part1<'a>(
    start: Cave<'a>,
    end: Cave<'a>,
    visited_caves: Vec<Cave<'a>>,
    map: &HashMap<Cave<'a>, Vec<Cave<'a>>>,
) -> Vec<Vec<Cave<'a>>> {
    let mut paths = Vec::new();
    let mut new_visited_caves = visited_caves.clone();
    new_visited_caves.push(start);
    if start == end {
        paths.push(new_visited_caves);
        return paths;
    }
    let adjacent_caves = map.get(&start).unwrap();
    for cave in adjacent_caves {
        if cave.small && visited_caves.contains(cave) {
            continue;
        }
        let new_visited_caves = new_visited_caves.clone();
        let new_paths = generate_paths_part1(*cave, end, new_visited_caves, map);
        for path in new_paths {
            paths.push(path);
        }
    }
    return paths;
}

fn generate_paths_part2<'a>(
    start: Cave<'a>,
    end: Cave<'a>,
    visited_caves: Vec<Cave<'a>>,
    map: &HashMap<Cave<'a>, Vec<Cave<'a>>>,
) -> Vec<Vec<Cave<'a>>> {
    let mut paths = Vec::new();
    let mut new_visited_caves = visited_caves.clone();
    new_visited_caves.push(start);
    if start == end {
        paths.push(new_visited_caves);
        return paths;
    }
    let adjacent_caves = map.get(&start).unwrap();
    let can_visit_another = can_visit_another_small_cave(&new_visited_caves);
    if !can_visit_another {
        for cave in adjacent_caves {
            if cave.small && visited_caves.contains(cave) {
                continue;
            }
            let new_visited_caves = new_visited_caves.clone();
            let new_paths = generate_paths_part2(*cave, end, new_visited_caves, map);
            for path in new_paths {
                paths.push(path);
            }
        }
        return paths;
    } else {
        for cave in adjacent_caves {
            if cave.label == "start" {
                continue;
            }
            let new_visited_caves = new_visited_caves.clone();
            let new_paths = generate_paths_part2(*cave, end, new_visited_caves, map);
            for path in new_paths {
                paths.push(path);
            }
        }
        return paths;
    }
}

fn can_visit_another_small_cave(visited_caves: &Vec<Cave>) -> bool {
    let mut map = HashMap::new();
    for cave in visited_caves
        .iter()
        .filter(|c| c.small && c.label != "start" && c.label != "end")
    {
        *map.entry(cave).or_insert(0) += 1;
    }
    map.values().filter(|v| **v > 1).count() == 0
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Cave<'a> {
    label: &'a str,
    small: bool,
}

#[cfg(test)]
mod tests_day12 {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW";
        let result = part1(input);
        assert_eq!(result, 226);
    }

    #[test]
    fn test_part2() {
        let input = "fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW";
        let result = part2(input);
        assert_eq!(result, 103);
    }
}
