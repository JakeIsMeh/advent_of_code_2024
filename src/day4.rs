use regex::Regex;
use std::fs::read_to_string;

pub fn solve(part: i32, data_path: &str) {
    let mut acc = 0;
    let mut transposed: Vec<String> = Vec::new();
    let mut rot1: Vec<String> = Vec::new();
    let mut rot2: Vec<String> = Vec::new();

    let mut first = true;

    let input = read_to_string(data_path).unwrap();
    let lines_itor = input.lines();
    let orig = lines_itor.clone().collect::<Vec<&str>>();
    let width = lines_itor.clone().next().unwrap().len() as i32;
    let height = lines_itor.clone().count() as i32;

    // let matcher = Regex::new(r"XMAS|SAMX").unwrap();
    let matcher = Regex::new(r"(XMAS|SAMX)").unwrap();

    // transpose the text while we iterate through it horizontally
    for line in lines_itor {
        if (first) {
            transposed.reserve(line.len());
            for _ in line.char_indices() {
                transposed.push(String::with_capacity((height + 4) as usize));
            }
            first = false;
        }

        let mut char_buf: [u8; 4] = [0; 4];
        for i in line.char_indices() {
            let tmp: &str = i.1.encode_utf8(&mut char_buf);
            transposed[i.0] += tmp;
        }

        acc += match_overlapping(&matcher, line);
    }

    // iterate through transposed text to find "vertically"
    for line in &transposed {
        acc += match_overlapping(&matcher, line);
    }

    // --- DIAGONALS ---
    // ASSUMES SQUARE INPUT
    rot1.reserve((height + (width - 1)) as usize);
    rot2.reserve((width + (height - 1)) as usize);

    let mut start_row: i32 = 0;
    let mut start_col: i32 = 0;

    // --- TOP LEFT CORNER TOP DIAGONAL ---
    while (start_row < height || start_col < width) {
        let mut curr_row: i32 = start_row.clamp(0, height - 1);
        let mut curr_col: i32 = start_col.clamp(0, width - 1);

        let mut diag_row = String::with_capacity(acc);
        while (curr_row >= 0 && curr_col < width) {
            diag_row += &orig[curr_row as usize]
                .chars()
                .nth(curr_col as usize)
                .unwrap()
                .to_string();

            curr_row -= 1;
            curr_col += 1;
        }

        rot1.push(diag_row);

        start_row += 1;
        if (start_row >= height) {
            start_col += 1;
        }
    }

    // --- TOP RIGHT CORNER TOP DIAGONAL ---
    start_row = 0;
    start_col = height - 1;

    while (start_row < height || start_col >= 0) {
        let mut curr_row: i32 = start_row.clamp(0, height - 1);
        let mut curr_col: i32 = start_col.clamp(0, width - 1);

        let mut diag_row = String::with_capacity(acc);
        while (curr_row >= 0 && curr_col >= 0) {
            diag_row += &orig[curr_row as usize].chars().nth(curr_col as usize).unwrap().to_string();

            curr_row -= 1;
            curr_col -= 1;
        }

        rot2.push(diag_row);

        start_row += 1;
        if (start_row >= height) {
            start_col -= 1;
        }
    }

    for line in rot1 {
        acc += match_overlapping(&matcher, &line);

    }

    for line in rot2 {
        acc += match_overlapping(&matcher, &line);

    }

    println!("count: {}", acc);
}

fn match_overlapping(matcher: &Regex, input: &str) -> usize {
    let mut acc = 0;
    let mut start: usize = 0;
    while (start < input.len()) {
        if let Some(m) = matcher.find_at(input, start) {
            start = m.start() + 1;
            acc += 1;
        } else {
            start += 1;
        }
    }

    return acc;
}
