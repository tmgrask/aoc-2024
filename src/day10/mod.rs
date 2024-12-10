use std::{collections::HashSet, fs};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
    value: u32,
}

struct Map {
    grid: Vec<Vec<Point>>,
}

impl Map {
    fn new(content: &str) -> Self {
        let mut grid = Vec::new();

        for (y, line) in content.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                row.push(Point {
                    x,
                    y,
                    value: c.to_digit(10).unwrap(),
                });
            }
            grid.push(row);
        }

        Map { grid }
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }
    fn height(&self) -> usize {
        self.grid.len()
    }
}

fn trailhead_score<'a>(map: &'a Map, start: &'a Point, reached: &mut HashSet<&'a Point>) -> i64 {
    let mut score = 0;

    if start.x > 0 {
        let left = &map.grid[start.y][start.x - 1];
        if left.value == start.value + 1 {
            score += trailhead_score(&map, &left, reached);
        }
    }

    if start.x < map.width() - 1 {
        let right = &map.grid[start.y][start.x + 1];
        if right.value == start.value + 1 {
            score += trailhead_score(&map, &right, reached);
        }
    }

    if start.y > 0 {
        let up = &map.grid[start.y - 1][start.x];
        if up.value == start.value + 1 {
            score += trailhead_score(&map, &up, reached);
        }
    }

    if start.y < map.height() - 1 {
        let down = &map.grid[start.y + 1][start.x];
        if down.value == start.value + 1 {
            score += trailhead_score(&map, &down, reached);
        }
    }

    if start.value == 9 && !reached.contains(start) {
        reached.insert(start);
        score += 1;
    }

    score
}

fn trailhead_rating(map: &Map, start: &Point) -> i64 {
    let mut score = 0;

    if start.x > 0 {
        let left = &map.grid[start.y][start.x - 1];
        if left.value == start.value + 1 {
            score += trailhead_rating(&map, &left);
        }
    }

    if start.x < map.width() - 1 {
        let right = &map.grid[start.y][start.x + 1];
        if right.value == start.value + 1 {
            score += trailhead_rating(&map, &right);
        }
    }

    if start.y > 0 {
        let up = &map.grid[start.y - 1][start.x];
        if up.value == start.value + 1 {
            score += trailhead_rating(&map, &up);
        }
    }

    if start.y < map.height() - 1 {
        let down = &map.grid[start.y + 1][start.x];
        if down.value == start.value + 1 {
            score += trailhead_rating(&map, &down);
        }
    }

    if start.value == 9 {
        score += 1;
    }

    score
}

pub fn part_one(file: &str) -> i64 {
    let content = fs::read_to_string(file).expect("File should exist and be readable to string");
    let map = Map::new(&content);

    let mut score = 0;

    for row in map.grid.iter() {
        for point in row {
            if point.value == 0 {
                let mut reached = HashSet::new();
                score += trailhead_score(&map, &point, &mut reached);
            }
        }
    }

    score
}

pub fn part_two(file: &str) -> i64 {
    let content = fs::read_to_string(file).expect("File should exist and be readable to string");
    let map = Map::new(&content);

    let mut score = 0;

    for row in map.grid.iter() {
        for point in row {
            if point.value == 0 {
                score += trailhead_rating(&map, &point);
            }
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let result = part_one(&"src/day10/test.txt");
        assert_eq!(result, 36);
    }

    #[test]
    fn p2() {
        let result = part_two(&"src/day10/test.txt");
        assert_eq!(result, 81);
    }
}
