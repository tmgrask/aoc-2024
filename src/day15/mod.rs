use std::{collections::HashMap, fmt, fs};

#[derive(Debug, Eq, PartialEq)]
enum Entity {
    Wall,
    Empty,
    Box,
    Robot,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Grid {
    coordinates: HashMap<Point, Entity>,
    robot: Point,
}

impl Grid {
    fn dimensions(&self) -> (usize, usize) {
        let mut max_x = self.robot.x;
        let mut max_y = self.robot.y;

        for point in self.coordinates.keys() {
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }

        (max_x + 1, max_y + 1)
    }

    fn next_point(&self, from: &Point, direction: &Direction) -> Point {
        match direction {
            Direction::Up => Point {
                x: from.x,
                y: from.y - 1,
            },
            Direction::Down => Point {
                x: from.x,
                y: from.y + 1,
            },
            Direction::Left => Point {
                x: from.x - 1,
                y: from.y,
            },
            Direction::Right => Point {
                x: from.x + 1,
                y: from.y,
            },
        }
    }
    fn box_moved(&mut self, point: &Point, direction: &Direction) -> bool {
        // recursively try to move box
        let next_point = self.next_point(point, direction);
        if let Some(next_entity) = self.coordinates.get_mut(&next_point) {
            match next_entity {
                Entity::Empty => {
                    self.coordinates.insert(next_point, Entity::Box);
                    true
                }
                Entity::Wall => false,
                Entity::Box => {
                    if self.box_moved(&next_point, direction) {
                        self.coordinates.insert(next_point, Entity::Box);
                        true
                    } else {
                        false
                    }
                }
                _ => panic!("Unexpected next entity"),
            }
        } else {
            panic!("Unexpected out of bounds!");
        }
    }

    fn move_robot_to(&mut self, point: &Point) {
        self.coordinates.insert(self.robot.clone(), Entity::Empty);
        self.coordinates.insert(point.clone(), Entity::Robot);
        self.robot = point.clone();
    }

    fn do_move(&mut self, direction: &Direction) {
        let next_point = self.next_point(&self.robot, direction);
        if let Some(next_entity) = self.coordinates.get_mut(&next_point) {
            match next_entity {
                Entity::Empty => {
                    self.move_robot_to(&next_point);
                }
                Entity::Wall => {}
                Entity::Box => {
                    if self.box_moved(&next_point, direction) {
                        self.move_robot_to(&next_point);
                    }
                }
                _ => panic!("Hit another robot??"),
            }
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (width, height) = self.dimensions();

        for y in 0..height {
            for x in 0..width {
                let point = Point { x, y };

                // Determine what character to print at this position
                let ch = if point == self.robot {
                    '@'
                } else {
                    match self.coordinates.get(&point) {
                        Some(Entity::Wall) => '#',
                        Some(Entity::Empty) => '·',
                        Some(Entity::Box) => 'O',
                        Some(Entity::Robot) => '@',
                        None => '·', // Default to empty for undefined positions
                    }
                };

                write!(f, "{}", ch)?;
            }
            writeln!(f)?; // New line at the end of each row
        }

        Ok(())
    }
}

fn load(content: &str) -> (Grid, Vec<Direction>) {
    let mut coordinates = HashMap::new();
    let mut moves = Vec::new();
    let mut robot = None;
    if let Some((grid_raw, moves_raw)) = content.split_once("\n\n") {
        for (y, line) in grid_raw.lines().enumerate() {
            for (x, c) in line.trim().chars().enumerate() {
                let point = Point { x, y };
                match c {
                    '#' => coordinates.insert(point, Entity::Wall),
                    '.' => coordinates.insert(point, Entity::Empty),
                    '@' => {
                        robot = Some(point.clone());
                        coordinates.insert(point, Entity::Robot)
                    }
                    'O' => coordinates.insert(point, Entity::Box),
                    c => panic!("unexpected char {c}"),
                };
            }
        }
        for moves_line in moves_raw.lines() {
            for move_raw in moves_line.trim().chars() {
                match move_raw {
                    '<' => moves.push(Direction::Left),
                    '>' => moves.push(Direction::Right),
                    '^' => moves.push(Direction::Up),
                    'v' => moves.push(Direction::Down),
                    c => panic!("unexpected char {c}"),
                };
            }
        }
    }

    (
        Grid {
            coordinates,
            robot: robot.expect("Grid must contain a robot"),
        },
        moves,
    )
}

pub fn part_one(file: &str) -> usize {
    let content = fs::read_to_string(file).expect("Should be readable");

    let (mut grid, moves) = load(&content);
    for next in moves {
        grid.do_move(&next);
        //println!("Move {:?}\n{}", next, grid);
    }

    let mut sum = 0;
    for (point, entity) in grid.coordinates {
        if entity == Entity::Box {
            sum += (point.y * 100) + point.x;
            //println!("sum={}, {:?}", sum, point);
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let small_result = part_one(&"src/day15/smalltest.txt");
        assert_eq!(small_result, 2028);

        let big_result = part_one(&"src/day15/bigtest.txt");
        assert_eq!(big_result, 10092);
    }
}
