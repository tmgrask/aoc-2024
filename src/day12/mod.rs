use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Side {
    val: i32,
    facing: Direction,
}

#[derive(Debug)]
struct Garden {
    map: HashMap<Point, char>,
    plants: HashSet<char>,
}

impl Garden {
    fn from(content: &str) -> Self {
        let mut map = HashMap::new();
        let mut plants = HashSet::new();
        for (y, line) in content.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let point = Point {
                    x: x as i32,
                    y: y as i32,
                };
                map.insert(point, c);
                plants.insert(c);
            }
        }

        Garden { map, plants }
    }

    fn cardinal_points_around(point: &Point) -> Vec<(Direction, Point)> {
        vec![
            (
                Direction::Left,
                Point {
                    x: point.x - 1,
                    y: point.y,
                },
            ),
            (
                Direction::Right,
                Point {
                    x: point.x + 1,
                    y: point.y,
                },
            ),
            (
                Direction::Up,
                Point {
                    x: point.x,
                    y: point.y - 1,
                },
            ),
            (
                Direction::Down,
                Point {
                    x: point.x,
                    y: point.y + 1,
                },
            ),
        ]
    }

    fn fill_contiguous_region_around<'a>(
        &'a self,
        point: &'a Point,
        c: &'a char,
        region: &mut HashSet<&'a Point>,
    ) {
        // insert the point we're on, return if we've already seen it
        if !region.insert(point) {
            return;
        }

        // check if we should recurse to the points in the cardinal directions
        for (_, next) in Garden::cardinal_points_around(point) {
            match self.map.get_key_value(&next) {
                Some((next_point, next_c)) => {
                    if next_c == c {
                        self.fill_contiguous_region_around(next_point, &c, region);
                    }
                }
                None => {}
            }
        }
    }

    #[allow(dead_code)]
    fn draw_region_with_perimeter(
        region: &HashSet<&Point>,
        perimeter: &HashSet<(Direction, Point)>,
    ) -> String {
        // Find the bounds of the region including perimeter
        let min_x = region
            .iter()
            .map(|p| p.x)
            .min()
            .unwrap_or(0)
            .min(perimeter.iter().map(|p| p.1.x).min().unwrap_or(0));
        let max_x = region
            .iter()
            .map(|p| p.x)
            .max()
            .unwrap_or(0)
            .max(perimeter.iter().map(|p| p.1.x).max().unwrap_or(0));
        let min_y = region
            .iter()
            .map(|p| p.y)
            .min()
            .unwrap_or(0)
            .min(perimeter.iter().map(|p| p.1.y).min().unwrap_or(0));
        let max_y = region
            .iter()
            .map(|p| p.y)
            .max()
            .unwrap_or(0)
            .max(perimeter.iter().map(|p| p.1.y).max().unwrap_or(0));

        // Create the grid with some padding
        let padding = 1;
        let width = (max_x - min_x + 1 + 2 * padding) as usize;
        let height = (max_y - min_y + 1 + 2 * padding) as usize;
        let mut grid = vec![vec![' '; width]; height];

        // Helper function to convert coordinates to grid indices
        let to_grid_coords = |x, y| {
            (
                (y - min_y + padding) as usize,
                (x - min_x + padding) as usize,
            )
        };

        // Fill in the region
        for point in region {
            let (row, col) = to_grid_coords(point.x, point.y);
            grid[row][col] = '·';
        }

        // Draw the perimeter
        for (_, point) in perimeter {
            let (row, col) = to_grid_coords(point.x, point.y);
            // Only draw perimeter if the spot isn't already part of the region
            if grid[row][col] == ' ' {
                grid[row][col] = 'o';
            }
        }

        // Convert grid to string
        grid.iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn perimiter_of(region: &HashSet<&Point>) -> usize {
        let mut perimeter = HashSet::new();
        for point in region {
            for (dir, next) in Garden::cardinal_points_around(point) {
                if !region.contains(&next) {
                    perimeter.insert((dir, next));
                }
            }
        }
        perimeter.len()
    }

    fn count_runs(runs: &HashSet<i32>) -> usize {
        let mut v = Vec::from_iter(runs);
        v.sort();
        let mut unique_runs = 0;
        let mut last = None;
        for i in v.clone() {
            if last == None {
                last = Some(i);
                unique_runs += 1;
            } else if i.clone() != last.unwrap() + 1 {
                last = Some(i);
                unique_runs += 1;
            } else {
                last = Some(i);
            }
        }

        unique_runs
    }

    fn number_of_sides(region: &HashSet<&Point>) -> usize {
        let mut perimeter = HashSet::new();
        for point in region {
            for (dir, next) in Garden::cardinal_points_around(point) {
                if !region.contains(&next) {
                    perimeter.insert((dir, next));
                }
            }
        }
        //println!("{}", Garden::draw_region_with_perimeter(region, &perimeter));

        let mut sides = HashMap::new();
        for (dir, point) in perimeter {
            match dir {
                Direction::Left => {
                    let side = Side {
                        val: point.x,
                        facing: Direction::Left,
                    };
                    let runs = sides.entry(side).or_insert_with(|| HashSet::new());
                    runs.insert(point.y);
                }
                Direction::Right => {
                    let side = Side {
                        val: point.x,
                        facing: Direction::Right,
                    };
                    let runs = sides.entry(side).or_insert_with(|| HashSet::new());
                    runs.insert(point.y);
                }
                Direction::Up => {
                    let side = Side {
                        val: point.y,
                        facing: Direction::Up,
                    };
                    let runs = sides.entry(side).or_insert_with(|| HashSet::new());
                    runs.insert(point.x);
                }
                Direction::Down => {
                    let side = Side {
                        val: point.y,
                        facing: Direction::Down,
                    };
                    let runs = sides.entry(side).or_insert_with(|| HashSet::new());
                    runs.insert(point.x);
                }
            };
        }

        let mut unique_sides = 0;
        for (_, runs) in sides.iter() {
            unique_sides += Garden::count_runs(runs);
        }

        unique_sides
    }

    fn price_of(&self, plant: &char) -> usize {
        let mut price = 0;
        let mut all_poi = HashSet::new();
        for (point, c) in self.map.iter().filter(|x| x.1 == plant) {
            // track the points we've seen, so we only process each once.
            if all_poi.contains(point) {
                continue;
            }

            // get the contiguous region that includes this point
            let mut region = HashSet::new();
            self.fill_contiguous_region_around(point, c, &mut region);

            // compute the perimiter and price
            let perimiter = Garden::perimiter_of(&region);
            let cost = perimiter * region.len();
            price += cost;

            // we don't need to re-process any of the points in the region
            for seen_point in region {
                all_poi.insert(seen_point);
            }
        }

        price
    }

    fn discounted_price_of(&self, plant: &char) -> usize {
        let mut price = 0;
        let mut all_poi = HashSet::new();
        for (point, c) in self.map.iter().filter(|x| x.1 == plant) {
            // track the points we've seen, so we only process each once.
            if all_poi.contains(point) {
                continue;
            }

            // get the contiguous region that includes this point
            let mut region = HashSet::new();
            self.fill_contiguous_region_around(point, c, &mut region);

            // compute the perimiter and price
            let number_of_sides = Garden::number_of_sides(&region);
            let cost = number_of_sides * region.len();
            price += cost;
            //println!(
            //    "A region of {} plants with price '{} * {} = {}'",
            //    c,
            //    region.len(),
            //    number_of_sides,
            //    cost
            //);

            // we don't need to re-process any of the points in the region
            for seen_point in region {
                all_poi.insert(seen_point);
            }
        }

        price
    }
}

pub fn part_one(file: &str) -> usize {
    let content = fs::read_to_string(file).expect("File should exist and be readable");

    let garden = Garden::from(&content);

    let mut price = 0;
    for plant in &garden.plants {
        price += garden.price_of(&plant);
    }

    price
}

pub fn part_two(file: &str) -> usize {
    let content = fs::read_to_string(file).expect("File should exist and be readable");

    let garden = Garden::from(&content);

    let mut price = 0;
    for plant in &garden.plants {
        price += garden.discounted_price_of(&plant);
    }

    price
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let example = part_one(&"src/day12/example.txt");
        assert_eq!(example, 140);
        let result = part_one(&"src/day12/test.txt");
        assert_eq!(result, 1930);
    }
    #[test]
    fn p2() {
        let mut run_single = HashSet::new();
        for i in vec![3, 4, 5, 6, 7] {
            run_single.insert(i);
        }
        assert_eq!(Garden::count_runs(&run_single), 1);

        let mut run_double = HashSet::new();
        for i in vec![3, 4, 6, 7] {
            run_double.insert(i);
        }
        assert_eq!(Garden::count_runs(&run_double), 2);

        let result = part_two(&"src/day12/test.txt");
        assert_eq!(result, 1206);
    }
}
