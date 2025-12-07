use itertools::Itertools;
use std::io::BufRead;
use range_set_blaze::prelude::*;

use std::{fs::File, io::BufReader};
fn main() -> std::io::Result<()> {
    let input_file = "myinput";
    let ranges = BufReader::new(File::open(input_file)?)
        .lines()
        .map_while(|l| l.ok())
        .take_while(|l| !l.is_empty())
        .map(|r: String| {
            let (start, end) = r.split_once('-').unwrap();
            start.parse::<i64>().unwrap()..=end.parse::<i64>().unwrap()
        })
        .collect_vec();

    let p1 = BufReader::new(File::open(input_file)?)
        .lines()
        .map_while(|l| l.ok())
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .map(|id| id.parse::<i64>().unwrap())
        .filter(|id| ranges.iter().find(|r| r.contains(id)).is_some())
        .count();

    println!("p2: {p1}");

    let ranges = BufReader::new(File::open(input_file)?)
        .lines()
        .map_while(|l| l.ok())
        .take_while(|l| !l.is_empty())
        .map(|r: String| {
            let (start, end) = r.split_once('-').unwrap();
            start.parse::<i64>().unwrap()..=end.parse::<i64>().unwrap()
        });

     let p2 = RangeSetBlaze::from_iter(ranges).len();

     println!("p2: {}", p2);

    Ok(())
}
