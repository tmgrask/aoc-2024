use std::{collections::HashSet, fmt, fs};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    obstacle: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cursor {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Cursor {
    fn turn(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<Point>>,
    cursor: Cursor,
    marked_points: HashSet<Point>,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let half_window = 20;

        let start_row = (self.cursor.y - half_window).max(0) as usize;
        let end_row = (self.cursor.y + half_window).min(self.grid.len() as i32 - 1) as usize;

        let row_width = self.grid[0].len();
        let start_col = (self.cursor.x - half_window).max(0) as usize;
        let end_col = (self.cursor.x + half_window).min(row_width as i32 - 1) as usize;

        for y in start_row..=end_row {
            if let Some(row) = self.grid.get(y) {
                for x in start_col..=end_col {
                    if self.marked_points.contains(&Point {
                        x: x as i32,
                        y: y as i32,
                        obstacle: false,
                    }) {
                        write!(f, "\x1b[32mO\x1b[0m")?;
                    } else if x as i32 == self.cursor.x && y as i32 == self.cursor.y {
                        match self.cursor.direction {
                            Direction::Up => write!(f, "\x1b[31m↑\x1b[0m")?,
                            Direction::Down => write!(f, "\x1b[31m↓\x1b[0m")?,
                            Direction::Left => write!(f, "\x1b[31m←\x1b[0m")?,
                            Direction::Right => write!(f, "\x1b[31m→\x1b[0m")?,
                        }
                    } else if row[x].obstacle {
                        write!(f, "#")?;
                    } else {
                        write!(f, "\x1b[90m·\x1b[0m")?;
                    }
                }
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl Map {
    fn new(content: &str) -> Self {
        let mut grid = Vec::new();

        let mut cursor: Option<Cursor> = None;

        for (y, line) in content.lines().enumerate() {
            let mut row = Vec::new();

            for (x, c) in line.split("").enumerate() {
                if c == "^" {
                    cursor = Some(Cursor {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                        direction: Direction::Up,
                    })
                } else if c == "v" {
                    cursor = Some(Cursor {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                        direction: Direction::Down,
                    })
                } else if c == ">" {
                    cursor = Some(Cursor {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                        direction: Direction::Right,
                    })
                } else if c == "<" {
                    cursor = Some(Cursor {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                        direction: Direction::Left,
                    })
                }

                row.push(Point {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                    obstacle: if c == "#" { true } else { false },
                })
            }

            grid.push(row);
        }

        Map {
            grid,
            cursor: cursor.expect("No Cursor encountered"),
            marked_points: HashSet::new(),
        }
    }

    // Take next step, returning the next point the cursor is on
    fn next(&mut self) -> Option<Point> {
        let (next_x, next_y) = match self.cursor.direction {
            Direction::Up => (self.cursor.x, self.cursor.y - 1),
            Direction::Down => (self.cursor.x, self.cursor.y + 1),
            Direction::Left => (self.cursor.x - 1, self.cursor.y),
            Direction::Right => (self.cursor.x + 1, self.cursor.y),
        };

        match self
            .grid
            .get(next_y as usize)
            .and_then(|row| row.get(next_x as usize))
            .cloned()
        {
            Some(next_point) if next_point.obstacle => {
                self.cursor.turn();
                self.next()
            }
            Some(next_point) => {
                self.cursor.x = next_x;
                self.cursor.y = next_y;
                Some(next_point)
            }
            None => None,
        }
    }

    fn simulate_obstruction(&mut self, patrolled_points: &HashSet<Point>) -> Option<Point> {
        // Add new obstruction point at the current cursor
        let new_obstacle = match self
            .grid
            .get(self.cursor.y as usize)
            .and_then(|row| row.get(self.cursor.x as usize))
            .cloned()
        {
            Some(point) => {
                if !point.obstacle {
                    Some(point)
                } else {
                    None
                }
            }
            None => None,
        };

        // Can't place on starting position
        if patrolled_points.contains(&new_obstacle.clone().unwrap()) {
            return None;
        }

        // Update the point in the map to actually be an obstacle
        let check_point = self
            .grid
            .get_mut(self.cursor.y as usize)
            .and_then(|row| row.get_mut(self.cursor.x as usize))
            .unwrap();
        check_point.obstacle = true;

        // backup 1 step depending on direction, then simulate with added obstacle
        match self.cursor.direction {
            Direction::Up => self.cursor.y += 1,
            Direction::Down => self.cursor.y -= 1,
            Direction::Left => self.cursor.x += 1,
            Direction::Right => self.cursor.x -= 1,
        };

        // if we return to a cursor position we've already seen during
        // simulation, we're in an infinite loop
        let mut simulated_cursors = HashSet::new();
        simulated_cursors.insert(self.cursor.clone());

        while let Some(_) = self.next() {
            if simulated_cursors.contains(&self.cursor) {
                return new_obstacle;
            } else {
                simulated_cursors.insert(self.cursor.clone());
            }
        }

        // if we get out of the loop, we left the map
        None
    }
}

fn deduplicate_points(points: Vec<Point>) -> Vec<Point> {
    let set: HashSet<_> = points.into_iter().collect();
    set.into_iter().collect()
}

pub fn part_one() -> usize {
    let content = fs::read_to_string("src/day06/input.txt").expect("Should read");

    let mut map = Map::new(content.as_str());

    let mut patrolled_points = Vec::new();
    patrolled_points.push(map.grid[map.cursor.y as usize][map.cursor.x as usize].clone());

    while let Some(patrolled_point) = map.next() {
        patrolled_points.push(patrolled_point);
    }

    deduplicate_points(patrolled_points).len()
}

pub fn part_two() -> usize {
    let content = fs::read_to_string("src/day06/input.txt").expect("Should read");

    let mut map = Map::new(content.as_str());

    let cursor_start = map.cursor.clone();

    // Record the start position so we don't try to place an obstacle there in the future
    let mut patrolled_points = HashSet::new();
    patrolled_points.insert(Point {
        x: cursor_start.x,
        y: cursor_start.y,
        obstacle: false,
    });

    // then iterate over each step, checking if adding an obstruction after the step makes a loop
    // keep track of positions we've walked on so we don't place obstacles there if we ever return
    let mut patrolled_points = HashSet::new();
    while let Some(patrolled_point) = map.next() {
        match map.clone().simulate_obstruction(&patrolled_points) {
            Some(obstruction_at) => {
                map.marked_points.insert(obstruction_at);
            }
            None => {}
        };
        patrolled_points.insert(patrolled_point);
    }

    map.marked_points.len()
}
