use std::{collections::HashSet, fs};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Antenna {
    symbol: String,
    x: i32,
    y: i32,
}

fn get_pairs(antennae: &Vec<Antenna>) -> Vec<(&Antenna, &Antenna)> {
    let mut pairs = Vec::new();
    for (i, antenna) in antennae.iter().enumerate() {
        for other_antenna in &antennae[i + 1..] {
            if antenna.symbol == other_antenna.symbol {
                pairs.push((antenna, other_antenna));
            }
        }
    }

    return pairs;
}

pub fn part_one(file: &str) -> usize {
    let content = fs::read_to_string(file).expect("Should read");

    // calculate bounds
    let lines: Vec<&str> = content.lines().collect();
    let y_size = lines.len() as i32;
    let x_size = lines[0].trim().chars().count() as i32;

    // Gather all antennae
    let mut antennae = Vec::new();

    for (y, line) in content.lines().enumerate() {
        for (x, symbol) in line.trim().chars().enumerate() {
            if symbol != '.' {
                antennae.push(Antenna {
                    symbol: symbol.to_string(),
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    // Find antinodes for each pair
    let mut antinodes = HashSet::new();
    for (a, b) in get_pairs(&antennae) {
        let mut new_nodes = Vec::new();
        new_nodes.push(Antenna {
            symbol: "#".to_string(),
            x: 2 * b.x - a.x,
            y: 2 * b.y - a.y,
        });
        new_nodes.push(Antenna {
            symbol: "#".to_string(),
            x: 2 * a.x - b.x,
            y: 2 * a.y - b.y,
        });

        for node in new_nodes {
            if node.x >= 0 && node.x < x_size && node.y >= 0 && node.y < y_size {
                antinodes.insert(node);
            }
        }
    }

    antinodes.len()
}

pub fn part_two(file: &str) -> usize {
    let content = fs::read_to_string(file).expect("Should read");

    // calculate bounds
    let lines: Vec<&str> = content.lines().collect();
    let y_size = lines.len() as i32;
    let x_size = lines[0].trim().chars().count() as i32;

    // Gather all antennae
    let mut antennae = Vec::new();

    for (y, line) in content.lines().enumerate() {
        for (x, symbol) in line.trim().chars().enumerate() {
            if symbol != '.' {
                antennae.push(Antenna {
                    symbol: symbol.to_string(),
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    // Find antinodes for each pair
    let mut antinodes = HashSet::new();
    for (a, b) in get_pairs(&antennae) {
        // Each node is it's pair's antinode
        antinodes.insert(Antenna {
            symbol: "#".to_string(),
            x: a.x,
            y: a.y,
        });
        antinodes.insert(Antenna {
            symbol: "#".to_string(),
            x: b.x,
            y: b.y,
        });
        // Calculate the "step" vector between points
        let dx = b.x - a.x;
        let dy = b.y - a.y;

        // Start from each antenna and keep stepping outward until we hit bounds
        // Direction 1: Starting from b, stepping away from a
        let mut x = b.x;
        let mut y = b.y;
        while x >= 0 && x < x_size && y >= 0 && y < y_size {
            x += dx;
            y += dy;
            if x >= 0 && x < x_size && y >= 0 && y < y_size {
                antinodes.insert(Antenna {
                    symbol: "#".to_string(),
                    x,
                    y,
                });
            }
        }

        // Direction 2: Starting from a, stepping away from b
        let mut x = a.x;
        let mut y = a.y;
        while x >= 0 && x < x_size && y >= 0 && y < y_size {
            x -= dx;
            y -= dy;
            if x >= 0 && x < x_size && y >= 0 && y < y_size {
                antinodes.insert(Antenna {
                    symbol: "#".to_string(),
                    x,
                    y,
                });
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let result = part_one(&"src/day08/test.txt");
        assert_eq!(result, 14);
    }

    #[test]
    fn p2() {
        let result = part_two(&"src/day08/test.txt");
        assert_eq!(result, 34);
    }
}
