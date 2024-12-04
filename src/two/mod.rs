use std::fs;

fn difference_is_ok(current: &i32, previous: &i32) -> bool {
    let difference = current - previous;
    if difference.abs() < 1 || difference.abs() > 3 {
        return false;
    }

    true
}

fn parse_report(report: &str) -> Vec<i32> {
    let mut parsed = Vec::new();
    for level in report.split(" ") {
        parsed.push(level.parse::<i32>().unwrap());
    }

    parsed
}

fn first_bad_index(report: &Vec<i32>) -> Option<usize> {
    let mut previous_level: Option<&i32> = None;
    let mut ascending_state: Option<bool> = None;

    for (i, level) in report.iter().enumerate() {
        match previous_level {
            Some(previous) => match ascending_state {
                Some(ascending) => match difference_is_ok(level, previous) {
                    true => match ascending {
                        true => {
                            if level < previous {
                                return Some(i);
                            }
                        }
                        false => {
                            if level > previous {
                                return Some(i);
                            }
                        }
                    },
                    false => return Some(i),
                },
                None => {
                    // This None case means it's the second value seen
                    if difference_is_ok(level, previous) {
                        if level > previous {
                            ascending_state = Some(true);
                        } else {
                            ascending_state = Some(false);
                        }
                    } else {
                        return Some(i);
                    }
                }
            },
            None => {
                // This None case means it's the very first value seen
                // We'll update previous_level below
            }
        }
        previous_level = Some(level);
    }

    None
}

pub fn part_one() -> i32 {
    let content = fs::read_to_string("src/two/input.txt").expect("Should read");

    let mut safe_reports = 0;

    for report_raw in content.lines() {
        let report = parse_report(report_raw);
        let unsafe_index = first_bad_index(&report);
        match unsafe_index {
            Some(_) => {}
            None => safe_reports += 1,
        }
    }

    safe_reports
}

pub fn part_two() -> i32 {
    let content = fs::read_to_string("src/two/input.txt").expect("Should read");

    let mut safe_reports = 0;

    for report_raw in content.lines() {
        let report = parse_report(report_raw);
        // first check if there's a bad level and remove it
        let unsafe_index = first_bad_index(&report);
        match unsafe_index {
            Some(index) => {
                // Need to try removing index and index-1 as either one may be the culprit
                let mut try_remove = vec![index - 1, index];
                // Also try removing the first index if the first unsafe index is at position 2
                if index == 2 {
                    try_remove.insert(0, index - 2);
                }
                for i in try_remove {
                    let mut report_cloned = report.clone();
                    let _removed = report_cloned.remove(i);
                    let still_unsafe = first_bad_index(&report_cloned);
                    match still_unsafe {
                        Some(_) => {
                            //println!("{report_raw}: Unsafe even after removing position {i}, {removed}.");
                        }
                        None => {
                            //println!("{report_raw}: Safe by removing level {i}, {removed}");
                            safe_reports += 1;
                            break;
                        }
                    }
                }
            }
            None => {
                //println!("{report_raw}: Safe without removing any level.");
                safe_reports += 1;
            }
        }
    }

    safe_reports
}
