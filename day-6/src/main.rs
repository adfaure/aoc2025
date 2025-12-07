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

    let matrix = pb
        .iter()
        .take(pb.len() - 1)
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut transposed: Vec<Vec<char>> = vec![vec!['.'; matrix.len()]; matrix[0].len()];

    for j in 0..matrix.len() {
        for i in 0..matrix[j].len() {
            transposed[i][j] = matrix[j][i];
        }
    }

    let p2 = transposed
        .iter()
        .map(|line| line.iter().filter(|c| **c != ' ').collect::<String>())
        .batching(|it| match it.next() {
            None => None,
            Some(c) => {
                let mut res = vec![c.parse::<i64>().unwrap()];
                for c in it.by_ref() {
                    if c.is_empty() {
                        break;
                    }
                    res.push(c.parse::<i64>().unwrap());
                }
                Some(res)
            }
        })
        .enumerate()
        .map(|(op, numbers)| {
            numbers.into_iter().reduce(|acc, n| match operations[op] {
                "*" => n * acc,
                "+" => n + acc,
                _ => unreachable!(),
            }).unwrap()
        })
        .sum::<i64>();

    println!("p2: {p2}");

    Ok(())
}
