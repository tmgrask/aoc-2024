use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Robot {
    p: Coord,
    v: Coord,
}

fn get_robots(content: &str) -> (Vec<Robot>, Coord) {
    let pattern = Regex::new(r"p=(?P<px>\d+),(?P<py>\d+)\s+v=(?P<vx>-?\d+),(?P<vy>-?\d+)").unwrap();

    let mut robots = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for line in content.lines() {
        if let Some(caps) = pattern.captures(line) {
            let px: i32 = caps.name("px").unwrap().as_str().parse().unwrap();
            let py: i32 = caps.name("py").unwrap().as_str().parse().unwrap();
            let vx: i32 = caps.name("vx").unwrap().as_str().parse().unwrap();
            let vy: i32 = caps.name("vy").unwrap().as_str().parse().unwrap();

            robots.push(Robot {
                p: Coord { x: px, y: py },
                v: Coord { x: vx, y: vy },
            });

            if px > max_x {
                max_x = px;
            }

            if py > max_y {
                max_y = py;
            }
        }
    }

    let dimensions = Coord {
        x: max_x + 1,
        y: max_y + 1,
    };

    (robots, dimensions)
}

fn safety_factor(robots: &Vec<Robot>, dimensions: &Coord, seconds: i32) -> (Vec<Vec<i32>>, i32) {
    let mut quadrants = vec![0, 0, 0, 0];

    let mut grid = Vec::new();

    for _ in 0..dimensions.y {
        let mut row = Vec::new();
        for _ in 0..dimensions.x {
            row.push(0);
        }
        grid.push(row);
    }

    for robot in robots {
        let x_res = (robot.v.x * seconds) % dimensions.x;
        let y_res = (robot.v.y * seconds) % dimensions.y;

        let x_disp = robot.p.x + x_res;
        let y_disp = robot.p.y + y_res;

        let x_final;
        let y_final;

        if x_disp >= dimensions.x {
            x_final = x_disp % dimensions.x;
        } else if x_disp < 0 {
            x_final = dimensions.x + x_disp;
        } else {
            x_final = x_disp;
        }

        if y_disp >= dimensions.y {
            y_final = y_disp % dimensions.y;
        } else if y_disp < 0 {
            y_final = dimensions.y + y_disp;
        } else {
            y_final = y_disp;
        }

        if x_final > (dimensions.x / 2) {
            if y_final > (dimensions.y / 2) {
                // bottom right
                quadrants[3] += 1;
            } else if y_final < (dimensions.y / 2) {
                // top right
                quadrants[1] += 1;
            }
        } else if x_final < (dimensions.x / 2) {
            if y_final > (dimensions.y / 2) {
                // bottom left
                quadrants[2] += 1;
            } else if y_final < (dimensions.y / 2) {
                // top left
                quadrants[0] += 1;
            }
        }

        grid[y_final as usize][x_final as usize] += 1;
    }

    let mut safety_factor = 0;
    for quadrant in quadrants {
        if safety_factor == 0 {
            safety_factor = quadrant;
        } else {
            safety_factor *= quadrant;
        }
    }

    (grid, safety_factor)
}

pub fn part_one(file: &str) -> i32 {
    let content = fs::read_to_string(file).expect("Should be readable");

    let (robots, dimensions) = get_robots(&content);

    let (_, sf) = safety_factor(&robots, &dimensions, 100);

    sf
}

fn save_grid_as_svg(grid: &Vec<Vec<i32>>, filename: &str, cell_size: u32) {
    let width = grid[0].len() as u32 * cell_size;
    let height = grid.len() as u32 * cell_size;

    // First, find the maximum value in the grid to scale colors properly
    let max_value = grid
        .iter()
        .flat_map(|row| row.iter())
        .max()
        .copied()
        .unwrap_or(1);

    let mut svg = String::from(format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" style="background-color: white">"#,
        width, height
    ));

    for (y, row) in grid.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            // Scale the value to 0-255 range based on the maximum value
            let color_value = if value <= 0 {
                255_u8
            } else {
                // This will make values closer to max_value appear darker
                255_u8.saturating_sub(((value as f32 / max_value as f32) * 255.0) as u8)
            };

            // Only draw non-zero values
            if value > 0 {
                svg.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="rgb({},{},{})" />"#,
                    x as u32 * cell_size,
                    y as u32 * cell_size,
                    cell_size,
                    cell_size,
                    color_value,
                    color_value,
                    color_value
                ));
            }
        }
    }

    svg.push_str("</svg>");

    let mut file = File::create(filename).expect("Failed to create file");
    file.write_all(svg.as_bytes()).expect("Failed to write SVG");
}

pub fn part_two(file: &str) -> i32 {
    // if "most of the robots" arrange themselves into a picture, the safety factor should be low?
    let content = fs::read_to_string(file).expect("Should be readable");

    let (robots, dimensions) = get_robots(&content);

    let mut min_sf = None;

    // Guess it happens within the first 10k iterations?
    for t in 1..10000 {
        let (_, sf) = safety_factor(&robots, &dimensions, t);
        if let Some((_, curr_min)) = min_sf {
            if curr_min > sf {
                min_sf = Some((t, sf));
            }
        } else {
            min_sf = Some((t, sf));
        }
    }

    let easter_t = min_sf.unwrap().0;

    let (grid, _) = safety_factor(&robots, &dimensions, easter_t);

    save_grid_as_svg(&grid, &"src/day14/easter-egg.svg", 4);

    easter_t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let result = part_one(&"src/day14/test.txt");
        assert_eq!(result, 12);
    }
}
