/**
 * --- PART 1 ---
 * The unusual data (your puzzle input) consists of many reports, one report per line. Each report is a list of numbers
 * called levels that are separated by spaces.
 * For example:
 *
 * 7 6 4 2 1
 * 1 2 7 8 9
 * 9 7 6 2 1
 * 1 3 2 4 5
 * 8 6 4 4 1
 * 1 3 6 7 9
 * This example data contains six reports each containing five levels.
 *
 * The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety systems can only tolerate
 * levels that are either gradually increasing or gradually decreasing. So, a report only counts as safe if both of the
 * following are true:
 *
 * The levels are either all increasing or all decreasing.
 * Any two adjacent levels differ by at least one and at most three.
 * In the example above, the reports can be found safe or unsafe by checking those rules:
 *
 * 7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
 * 1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
 * 9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
 * 1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
 * 8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
 * 1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
 * So, in this example, 2 reports are safe.
 *
 * Analyze the unusual data from the engineers. How many reports are safe?
 *
 * --- PART 2 ---
 * Now, the same rules apply as before, except if removing a single level from an unsafe report would make it safe, the
 * report instead counts as safe.
 *
 * More of the above example's reports are now safe:
 *
 * 7 6 4 2 1: Safe without removing any level.
 * 1 2 7 8 9: Unsafe regardless of which level is removed.
 * 9 7 6 2 1: Unsafe regardless of which level is removed.
 * 1 3 2 4 5: Safe by removing the second level, 3.
 * 8 6 4 4 1: Safe by removing the third level, 4.
 * 1 3 6 7 9: Safe without removing any level.
 * Thanks to the Problem Dampener, 4 reports are actually safe!
 *
 * Update your analysis by handling situations where the Problem Dampener can remove a single level from unsafe reports.
 * How many reports are now safe?
 */
use std::fs::read_to_string;

pub fn solve(part: i32, data_path: &str) {
    println!("Solving Day 2...");

    let mut data: Vec<Vec<i32>> = Vec::new();
    for line in read_to_string(data_path).unwrap().lines() {
        let line_parse_itor = line
            .split_whitespace()
            .filter(|&s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap());

        let res: Vec<i32> = line_parse_itor.collect();

        if (res.len() < 2) {
            panic!("Line had too little data");
        }

        data.push(res);
    }

    let mut unsafe_data: Vec<Vec<i32>> = Vec::new();

    let mut safe_acc = 0;
    for line in data {
        if (part1_line(&line)) {
            safe_acc += 1;
        } else {
            unsafe_data.push(line);
        }
    }

    if (part == 0 || part == 1) {
        println!("Part 1 Ans: {}", safe_acc);
    }

    if (part == 0 || part == 2) {
        let mut salvages = 0;
        for line in unsafe_data {
            if (part2_line(&line)) {
                salvages += 1;
            }
        }

        println!("Part 2 Ans: {}", safe_acc + salvages);
    }
}

fn part1_line(data: &[i32]) -> bool {
    let initial_dir = (data[1] - data[0]).signum();

    // 0 means the first two levels don't differ, which is unsafe.
    if (initial_dir == 0) {
        return false;
    }

    // starts from 0 so that first number delta is checked
    for i in 0..(data.len() - 1) {
        let delta = (data[i + 1] - data[i]) * initial_dir;
        if (!(1..=3).contains(&delta)) {
            return false;
        }
    }

    return true;
}

fn part2_line(data: &[i32]) -> bool {
    for i in 0..data.len() {
        let mut tmp = data.to_vec();
        tmp.remove(i);
        if (part1_line(&tmp)) {
            return true;
        }
    }
    return false;
}
