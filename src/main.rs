use std::{path::PathBuf, process::exit};

use clap::{arg, value_parser, Command};

mod day1;
mod day2;
mod day3;
mod day4;

fn main() -> Result<(), std::io::Error> {
    let matches = Command::new("AoC 2024") // requires `cargo` feature
        .version("0.0")
        .arg(
            arg!(<day> "day's puzzle to run")
                .value_parser(value_parser!(i32))
                .required(true),
        )
        .arg(
            arg!([part])
                .value_parser(value_parser!(i32))
                .default_value("1"),
        )
        .arg(
            arg!(
                -d --data <file> "Use a custom data input"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let day = *(matches.get_one::<i32>("day").expect("day is missing"));

    let part = *(matches.get_one::<i32>("part").expect("part is missing"));
    let path: String;

    if let Ok(Some(p)) = matches.try_get_one::<PathBuf>("data") {
        path = p.to_string_lossy().to_string();
    } else {
        path = format!("./data/day{}.txt", day);
    }

    if !(..=2).contains(&part) {
        println!(r"Part should fall within 0-2. (0 for both parts)\n");
        exit(1);
    }

    match day {
        1 => day1::solve(part, &path),
        2 => day2::solve(part, &path),
        3 => day3::solve(part, &path),
        4 => day4::solve(part, &path),
        _ => println!("Day not implemented."),
    }

    return Ok(());
}
