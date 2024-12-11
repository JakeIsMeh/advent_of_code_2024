/**
 * --- PART 1 ---
 * 3   4
 * 4   3
 * 2   5
 * 1   3
 * 3   9
 * 3   3
 * Maybe the lists are only off by a small amount! To find out, pair up the numbers and measure how far apart they are.
 * Pair up the smallest number in the left list with the smallest number in the right list, then the second-smallest
 * left number with the second-smallest right number, and so on.
 *
 * Within each pair, figure out how far apart the two numbers are; you'll need to add up all of those distances. For
 * example, if you pair up a 3 from the left list with a 7 from the right list, the distance apart is 4; if you pair up
 * a 9 with a 3, the distance apart is 6.
 *
 * In the example list above, the pairs and distances would be as follows:
 * The smallest number in the left list is 1, and the smallest number in the right list is 3. The distance between them
 * is 2.
 * The second-smallest number in the left list is 2, and the second-smallest number in the right list is another 3. The
 * distance between them is 1.
 * The third-smallest number in both lists is 3, so the distance between them is 0.
 * The next numbers to pair up are 3 and 4, a distance of 1.
 * The fifth-smallest numbers in each list are 3 and 5, a distance of 2.
 * Finally, the largest number in the left list is 4, while the largest number in the right list is 9; these are a
 * distance 5 apart.
 *
 * To find the total distance between the left list and the right list, add up the distances between all of the pairs
 * you found. In the example above, this is 2 + 1 + 0 + 1 + 2 + 5, a total distance of 11!
 *
 * Your actual left and right lists contain many location IDs. What is the total distance between your lists?
 *
 * --- PART 2 ---
 * This time, you'll need to figure out exactly how often each number from the left list appears in the right list.
 * Calculate a total similarity score by adding up each number in the left list after multiplying it by the number of
 * times that number appears in the right list.
 *
 * Here are the same example lists again:
 *
 * 3   4
 * 4   3
 * 2   5
 * 1   3
 * 3   9
 * 3   3
 * For these example lists, here is the process of finding the similarity score:
 *
 * The first number in the left list is 3. It appears in the right list three times, so the similarity score increases by 3 * 3 = 9.
 * The second number in the left list is 4. It appears in the right list once, so the similarity score increases by 4 * 1 = 4.
 * The third number in the left list is 2. It does not appear in the right list, so the similarity score does not increase (2 * 0 = 0).
 * The fourth number, 1, also does not appear in the right list.
 * The fifth number, 3, appears in the right list three times; the similarity score increases by 9.
 * The last number, 3, appears in the right list three times; the similarity score again increases by 9.
 * So, for these example lists, the similarity score at the end of this process is 31 (9 + 4 + 0 + 0 + 9 + 9).
 *
 * Once again consider your left and right lists. What is their similarity score?
 */
use std::{array::from_fn, fs::read_to_string};

pub fn solve(part: i32, data_path: &str) {
    let mut lhs = Vec::<i32>::new();
    let mut rhs = Vec::<i32>::new();

    for line in read_to_string(data_path).unwrap().lines() {
        let mut line_parse_itor = line
            .split_whitespace()
            .filter(|&s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap());

        let nums: [i32; 2] = from_fn(|_| line_parse_itor.next().expect("oops"));

        lhs.push(nums[0]);
        rhs.push(nums[1]);
    }

    assert_eq!(lhs.len(), rhs.len());

    lhs.sort();
    rhs.sort();

    if (part == 0 || part == 1) {
        let mut p1_res = 0;

        for i in 0..lhs.len() {
            p1_res += (lhs[i] - rhs[i]).abs();
        }

        println!("Part 1 Ans: {}", p1_res);
    }

    if (part == 0 || part == 2) {
        let mut similarity_score = 0;

        let mut left_num_idx: usize = 0;

        loop {
            let right_start = rhs.partition_point(|&x| x < lhs[left_num_idx]);
            let right_end = rhs.partition_point(|&x| x <= lhs[left_num_idx]);

            let left_end = lhs.partition_point(|&x| x <= lhs[left_num_idx]);

            let right_n = right_end - right_start;
            let left_n = left_end - left_num_idx;

            similarity_score += lhs[left_num_idx] * (left_n as i32) * (right_n as i32);

            left_num_idx = left_end;

            if (left_num_idx >= lhs.len()) {
                break;
            }
        }

        println!("Part 2 Ans: {}", similarity_score);
    }

    return;
}