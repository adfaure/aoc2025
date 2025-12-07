use itertools::Itertools;
use std::io::BufRead;

use std::{fs::File, io::BufReader};
fn main() -> std::io::Result<()> {
    let input_file = "myinput";

    let pb = BufReader::new(File::open(input_file)?)
        .lines()
        .map_while(|l| l.ok())
        .collect_vec(); // lets collect this time

    let operations = pb.last().unwrap().split_ascii_whitespace().collect_vec();

    let p1 = pb
        .iter()
        .take(pb.len() - 1)
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|nb| nb.parse::<i64>().unwrap())
                .collect_vec()
        })
        .reduce(|acc, n| {
            acc.into_iter()
                .zip(n)
                .zip(operations.iter())
                .map(|((a, b), op)| match *op {
                    "*" => a * b,
                    "+" => a + b,
                    _ => unreachable!(),
                })
                .collect_vec()
        })
        .unwrap()
        .iter()
        .sum::<i64>();

    println!("p1: {p1:?}");

    Ok(())
}
