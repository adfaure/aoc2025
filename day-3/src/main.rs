use itertools::Itertools;
use std::io::BufRead;
use std::io::Read;

use std::{fs::File, io::BufReader};
// 16456
// 17264
fn main() -> std::io::Result<()> {
    let p1 = BufReader::new(File::open("myinput")?)
        .lines()
        .map_while(|l| l.ok())
        .map(|l| {
            let (max_index, max) = l
                .chars()
                .rev()
                .enumerate()
                .skip(1)
                .map(|(i, c): (usize, char)| (l.len() - (i + 1), c.to_digit(10).unwrap()))
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();

            let second = l
                .chars()
                .skip(max_index + 1)
                // .inspect(|e| println!("{e}"))
                .map(|c| c.to_digit(10).unwrap())
                .max().unwrap();

            let res = max * 10 + second;
            res

        }).sum::<u32>();

    println!("{p1:?}");

    Ok(())
}
