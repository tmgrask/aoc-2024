use std::fs;

fn load_blocks(content: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut id = 0;
    for (i, c) in content.trim().chars().enumerate() {
        let digit = c.to_digit(10).unwrap();
        if (i % 2) == 0 {
            for _ in 0..digit {
                blocks.push(id.to_string())
            }
        } else {
            for _ in 0..digit {
                blocks.push(".".to_string());
            }
            id += 1;
        }
    }

    blocks
}

fn get_last_block(blocks: &mut Vec<String>) -> Option<String> {
    if let Some(last) = blocks.pop() {
        if last == "." {
            return get_last_block(blocks);
        } else {
            return Some(last);
        }
    } else {
        None
    }
}

fn get_last_id_block(blocks: &mut Vec<String>) -> Option<(usize, Vec<String>)> {
    let mut last_id_block = Vec::new();
    match get_last_block(blocks) {
        Some(last_id) => {
            last_id_block.push(last_id.clone());
            let starting_length = blocks.len() + 1;
            while blocks.last() == Some(&last_id) {
                last_id_block.push(get_last_block(blocks).unwrap());
            }
            Some((starting_length - last_id_block.len(), last_id_block))
        }
        None => None,
    }
}

fn index_of_free_block(blocks: &Vec<String>, length: usize) -> Option<usize> {
    let mut index = 0;
    while index <= blocks.len() {
        let mut run_length = 0;
        let mut run_index = index.clone();
        while blocks[run_index] == "." {
            run_index += 1;
            run_length += 1;
            //println!(
            //    "run_index: {:?}, blocks len: {:?}, run_length: {:?}/{:?}",
            //    run_index,
            //    blocks.len(),
            //    run_length,
            //    length
            //);
            if run_index + 1 >= blocks.len() {
                //println!("running off end");
                return None;
            }
        }
        if run_length >= length {
            return Some(index);
        }
        run_index += 1;
        index = run_index;
        //println!("index: {:?}", index);
    }

    None
}

fn checksum(blocks: &Vec<String>) -> i64 {
    let mut total = 0;
    for (pos, id) in blocks.iter().enumerate() {
        if id == "." {
            continue;
        }
        total += (pos as i64) * id.parse::<i64>().unwrap();
    }
    total
}

pub fn part_one(file: &str) -> i64 {
    let content = fs::read_to_string(file).expect("File should exist and be readable to string");

    //println!("{:?}", blocks.join(""));
    let mut blocks = load_blocks(&content);

    while blocks.contains(&".".to_string()) {
        let last_block = get_last_block(&mut blocks);
        let first_blank_pos = blocks.iter().position(|c| c == ".");
        match last_block {
            Some(last) => match first_blank_pos {
                Some(pos) => blocks[pos] = last,
                None => blocks.push(last), // put it back
            },
            None => {}
        }
    }

    //println!("{:?}", blocks.join(""));

    checksum(&blocks)
}

pub fn part_two(file: &str) -> i64 {
    let content = fs::read_to_string(file).expect("File should exist and be readable to string");

    let mut blocks = load_blocks(&content);
    //println!("{:?}", blocks.join(""));

    let mut completed_ids = Vec::new();
    let mut cloned_blocks = blocks.clone();
    loop {
        if let Some((start, last_id_block)) = get_last_id_block(&mut cloned_blocks) {
            if completed_ids.contains(&last_id_block[0].clone()) {
                continue;
            }
            match index_of_free_block(&blocks, last_id_block.len()) {
                Some(insert_at) => {
                    if insert_at > start {
                        //println!(
                        //    "{:?} would move in the wrong direction, keep it",
                        //    last_id_block
                        //);
                        continue;
                    }
                    //println!(
                    //    "move {:?} from {:?} to {:?}",
                    //    last_id_block, start, insert_at
                    //);
                    blocks.splice(
                        insert_at..insert_at + last_id_block.len(),
                        last_id_block.iter().cloned(),
                    );
                    blocks.splice(
                        start..start + last_id_block.len(),
                        vec![".".to_string(); last_id_block.len()],
                    );
                }
                None => {
                    //println!("No space, keep {:?} where it was", last_id_block);
                    blocks.splice(start..start + last_id_block.len(), last_id_block.clone());
                }
            }
            completed_ids.push(last_id_block[0].clone());
        } else {
            break;
        }
        //println!("{:?}", blocks.join(""));
    }

    //println!("{:?}", blocks.join(""));

    checksum(&blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let result = part_one(&"src/day09/test.txt");
        assert_eq!(result, 1928);
    }

    #[test]
    fn p2() {
        let result = part_two(&"src/day09/test.txt");
        assert_eq!(result, 2858);
    }
}
