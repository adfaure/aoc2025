use itertools::*;
use std::io::Read;

use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    let mut input = Default::default();
    let _ = BufReader::new(File::open("myinput")?).read_to_string(&mut input);

    let p1 = input
        .trim()
        .split(',')
        .map(|e| {
            e.split('-')
                .map(|e| e.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .flat_map(|(min, max)| {
            (min..=max).filter(|n: &u64| {
                let mut nb_digit = n.ilog10() + 1;
                if *n == 0 {
                    nb_digit = 1;
                }
                let right = *n / 10_i32.pow(nb_digit / 2) as u64;
                let left = *n % 10_i32.pow(nb_digit / 2) as u64;
                right == left
            })
        })
        .sum::<u64>();

    println!("p1: {}", p1);
    Ok(())
}
