use itertools::Itertools;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

use std::cmp;

fn main() -> std::io::Result<()> {
    let input_file = "input";

    let p1 = BufReader::new(File::open(input_file)?)
        .lines()
        .map_while(|l| l.ok())
        .map(|l: String| {
            let (a, b) = l.split(",").collect_tuple().unwrap();
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .combinations(2)
        .map(|v| {
            let (a, b) = (v[0], v[1]);

            let res = if a.0 == b.0 {
                (a.1 - b.1).abs() + 1
            } else if a.1 == b.1 {
                (a.0 - b.0).abs() + 1
            } else {
                (1 + (a.0 - b.0).abs()) * (1 + (a.1 - b.1).abs())
            };
            (a, b, res)
        })
        .max_by(|a, b| a.2.cmp(&b.2))
        .unwrap();
    println!("p1: {}", p1.2);

    let shape = BufReader::new(File::open(input_file)?)
        .lines()
        .map_while(|l| l.ok())
        .map(|l: String| {
            let (a, b) = l.split(",").collect_tuple().unwrap();
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .collect_vec();

    let p2 = BufReader::new(File::open(input_file)?)
        .lines()
        .map_while(|l| l.ok())
        .map(|l: String| {
            let (a, b) = l.split(",").collect_tuple().unwrap();
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .combinations(2)
        .map(|v| v.into_iter().collect_tuple::<(_, _)>().unwrap())
        .map(|(a, b)| ((a.0, a.1), (b.0, a.1), (a.0, b.1), (b.0, b.1)))
        .filter(|(a, _, _, b)| {
            // Rectangle is a line
            if a.0 == b.0 || a.1 == b.1 {
                return true;
            }

            let (min_x, max_x) = (a.0.min(b.0), a.0.max(b.0));
            let (min_y, max_y) = (a.1.min(b.1), a.1.max(b.1));

            shape.iter().any(|coord| {
                min_x < coord.0 && coord.0 < max_x && min_y < coord.1 && coord.1 < max_y
            })

        })
        .collect_vec();

    println!("{:?}", p2.len());

    Ok(())
}
