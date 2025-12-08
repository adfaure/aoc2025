use itertools::Itertools;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn dist3d(a: (i64, i64, i64), b: (i64, i64, i64)) -> i64 {
    ((a.0 - b.0).abs().pow(2) + (a.1 - b.1).abs().pow(2) + (a.2 - b.2).abs().pow(2)).isqrt()
}

fn main() -> std::io::Result<()> {
    let input_file = "input";
    let p1 = BufReader::new(File::open(input_file)?)
        .lines()
        .map_while(|l| l.ok())
        .flat_map(|l| {
            l.split(",")
                .map(|str| str.parse::<i64>().unwrap())
                .collect_tuple::<(_, _, _)>()
        })
        .collect_vec();

    println!("p1: {:?}", p1);

    Ok(())
}
