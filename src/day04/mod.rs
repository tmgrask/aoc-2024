use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    fn get_center_position(&self, direction: Direction) -> Position {
        // Get the position of the 'A' (second letter) in MAS
        Position {
            row: self.row + direction.row_delta,
            col: self.col + direction.col_delta,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Direction {
    row_delta: i32,
    col_delta: i32,
}

#[derive(Debug)]
struct WordLocation {
    start: Position,
    direction: Direction,
}

struct WordFinder {
    grid: Vec<Vec<char>>,
    height: i32,
    width: i32,
    directions: Vec<Direction>,
}

impl WordFinder {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let height = grid.len() as i32;
        let width = if height > 0 { grid[0].len() as i32 } else { 0 };

        // Define all 8 possible directions
        let directions = vec![
            Direction {
                row_delta: 0,
                col_delta: 1,
            }, // right
            Direction {
                row_delta: 1,
                col_delta: 1,
            }, // down-right
            Direction {
                row_delta: 1,
                col_delta: 0,
            }, // down
            Direction {
                row_delta: 1,
                col_delta: -1,
            }, // down-left
            Direction {
                row_delta: 0,
                col_delta: -1,
            }, // left
            Direction {
                row_delta: -1,
                col_delta: -1,
            }, // up-left
            Direction {
                row_delta: -1,
                col_delta: 0,
            }, // up
            Direction {
                row_delta: -1,
                col_delta: 1,
            }, // up-right
        ];

        WordFinder {
            grid,
            height,
            width,
            directions,
        }
    }

    fn find_word(&self, word: &str) -> Vec<WordLocation> {
        let word: Vec<char> = word.to_uppercase().chars().collect();
        let mut results = Vec::new();

        // Try each starting position
        for row in 0..self.height {
            for col in 0..self.width {
                // Try each direction
                for &direction in &self.directions {
                    if self.check_word(&word, Position { row, col }, direction) {
                        results.push(WordLocation {
                            start: Position { row, col },
                            direction,
                        });
                    }
                }
            }
        }

        results
    }

    fn check_word(&self, word: &[char], start: Position, direction: Direction) -> bool {
        let word_len = word.len() as i32;

        // Check if word would go out of bounds
        let end_row = start.row + (word_len - 1) * direction.row_delta;
        let end_col = start.col + (word_len - 1) * direction.col_delta;

        if end_row < 0 || end_row >= self.height || end_col < 0 || end_col >= self.width {
            return false;
        }

        // Check each character
        let mut curr_pos = start;
        for &char in word {
            if self.grid[curr_pos.row as usize][curr_pos.col as usize] != char {
                return false;
            }
            curr_pos = Position {
                row: curr_pos.row + direction.row_delta,
                col: curr_pos.col + direction.col_delta,
            };
        }

        true
    }
}

fn find_intersections(locations: &[WordLocation]) -> Vec<Position> {
    let mut intersection_points = Vec::new();

    // Filter to only diagonal directions
    let diagonal_locations: Vec<&WordLocation> = locations
        .iter()
        .filter(|loc| {
            let d = loc.direction;
            d.row_delta.abs() == 1 && d.col_delta.abs() == 1
        })
        .collect();

    // Compare each pair of diagonal MAS
    for (i, loc1) in diagonal_locations.iter().enumerate() {
        for loc2 in diagonal_locations.iter().skip(i + 1) {
            // Get the position of the 'A' in each CAT
            let a1 = loc1.start.get_center_position(loc1.direction);
            let a2 = loc2.start.get_center_position(loc2.direction);

            // If the A's overlap, this is an intersection point
            if a1 == a2 {
                intersection_points.push(a1);
            }
        }
    }

    // Remove duplicates
    intersection_points.sort_by_key(|p| (p.row, p.col));
    intersection_points.dedup();

    intersection_points
}

fn prepare_grid(content: String) -> Vec<Vec<char>> {
    let mut grid = Vec::new();

    for row_raw in content.lines() {
        let mut row = Vec::new();
        for c in row_raw.split("") {
            if c == "" {
                continue;
            }
            row.push(c.chars().next().expect("Should be a char"));
        }
        grid.push(row);
    }

    grid
}

pub fn part_one() -> usize {
    let content = fs::read_to_string("src/day04/input.txt").expect("Should read");

    let word_finder = WordFinder::new(prepare_grid(content));

    let results = word_finder.find_word("XMAS");

    results.len()
}

pub fn part_two() -> usize {
    let content = fs::read_to_string("src/day04/input.txt").expect("Should read");

    let word_finder = WordFinder::new(prepare_grid(content));

    let results = word_finder.find_word("MAS");

    let intersections = find_intersections(&results);

    intersections.len()
}
