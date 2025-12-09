use itertools::Itertools;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    let input_file = "myinput";

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

    Ok(())
}
