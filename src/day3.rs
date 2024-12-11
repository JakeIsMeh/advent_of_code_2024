/**
 * --- PART 1 ---
 * The computer appears to be trying to run a program, but its memory (your puzzle input) is corrupted. All of the
 * instructions have been jumbled up!
 *
 * It seems like the goal of the program is just to multiply some numbers. It does that with instructions like mul(X,Y),
 * where X and Y are each 1-3 digit numbers. For instance, mul(44,46) multiplies 44 by 46 to get a result of 2024.
 * Similarly, mul(123,4) would multiply 123 by 4.
 *
 * However, because the program's memory has been corrupted, there are also many invalid characters that should be
 * ignored, even if they look like part of a mul instruction. Sequences like `mul(4*, mul(6,9!, ?(12,34)`, or `mul ( 2 , 4 )` do nothing.
 *
 * For example, consider the following section of corrupted memory:
 *
 * ```x`mul(2,4)`%&mul[3,7]!@^do_not_`mul(5,5)`+mul(32,64]then(`mul(11,8)``mul(8,5)`)```
 * Only the four highlighted sections are real mul instructions. Adding up the result of each instruction produces 161
 * `(2*4 + 5*5 + 11*8 + 8*5)`.
 *
 * Scan the corrupted memory for uncorrupted mul instructions. What do you get if you add up all of the results of the
 * multiplications?
 *
 * --- PART 2 ---
 *
 * There are two new instructions you'll need to handle:
 *
 * The do() instruction enables future mul instructions.
 * The don't() instruction disables future mul instructions.
 * Only the most recent do() or don't() instruction applies. At the beginning of the program, mul instructions are enabled.
 *
 * For example:
 *
 * x`mul(2,4)`&mul[3,7]!^`don't()`_mul(5,5)+mul(32,64](mul(11,8)un`do()`?`mul(8,5)`)
 * This corrupted memory is similar to the example from before, but this time the mul(5,5) and mul(11,8) instructions are disabled because there is a don't() instruction before them. The other mul instructions function normally, including the one at the end that gets re-enabled by a do() instruction.
 *
 * This time, the sum of the results is 48 (2*4 + 8*5).
 *
 * Handle the new instructions; what do you get if you add up all of the results of just the enabled multiplications?
 */
use std::fs::read_to_string;
use regex::Regex;

pub fn solve(part: i32, data_path: &str) {
    let input = read_to_string(data_path).unwrap();

    if (part == 0 || part == 1) {
        let res = part1(&input);
        println!("Part 1 Ans: {}", res);
    }

    if (part == 0 || part == 2) {
        let res = part2(&input);
        println!("Part 2 Ans: {}", res);
    }

}

fn part1(input: &str) -> i32 {
    let matcher = Regex::new(r"mul\((\d*)\,(\d*)\)").unwrap();
    let mul_pairs: Vec<(i32, i32)> = matcher
        .captures_iter(input)
        .map(|caps| {
            let (_, [lhs, rhs]) = caps.extract();
            (lhs.parse::<i32>().unwrap(), rhs.parse::<i32>().unwrap())
        })
        .collect();

    let mut sum = 0;
    for pair in mul_pairs {
        sum += pair.0 * pair.1;
    }

    return sum;
}

fn part2(input: &str) -> i32 {
    let matcher = Regex::new(r"(?:mul\((\d*)\,(\d*)\))|(don't\(\))|(do\(\))").unwrap();
    let mul_matcher = Regex::new(r"mul\((\d*)\,(\d*)\)").unwrap();

    let mut mul_pairs: Vec<(i32, i32)> = Vec::new();
    // INSTRUCTIONS: 0 - MUL, 1 - DISABLE, 2 - ENABLE
    let mut instr_stream: Vec<u8> = Vec::new();

    matcher.find_iter(input).for_each(|m| {
        let m_str = m.as_str();

        if (m_str.starts_with("mul")) {
            for (_, [lhs, rhs]) in mul_matcher.captures_iter(m_str).map(|c| c.extract()) {
                mul_pairs.push((lhs.parse::<i32>().unwrap(), rhs.parse::<i32>().unwrap()));
            }
            instr_stream.push(0);
        } else if (m_str.starts_with("don")) {
            instr_stream.push(1);
        } else if (m_str.starts_with("do")) {
            instr_stream.push(2);
        }
    });

    let mut ip = 0; // instruction pointer
    let mut dp = 0; // data pointer
    let mut enabled = true;
    let mut acc = 0;
    loop {
        match instr_stream[ip] {
            0 => {
                if (enabled) {
                    acc += mul_pairs[dp].0 * mul_pairs[dp].1;
                }
                dp += 1;
            },
            1 => {
                enabled = false;
            },
            2 => {
                enabled = true;
            },
            _ => (),
        }
        ip += 1;
        if (ip >= instr_stream.len()) { break; }
    }

    return acc;
}
