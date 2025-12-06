use itertools::Itertools;
use std::io::BufRead;

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
                .max()
                .unwrap();

            max * 10 + second
        })
        .sum::<u32>();

    println!("{p1:?}");

    let p2 = BufReader::new(File::open("myinput")?)
        .lines()
        .map_while(|l| l.ok())
        .map(|l| {
            let chars = l
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect_vec();

            let n = 12;
            let mut res = 0;
            let mut index = 0;
            let mut len = l.len();

            for i in (0..n).rev() {
                let max = chars.iter().skip(index).take(len - i).max().unwrap();

                index = chars
                    .iter()
                    .enumerate()
                    .skip(index)
                    .find(|(_idx, x)| *x == max)
                    .unwrap().0 + 1;

                len = l.len() - index;

                res += *max * 10_u64.pow(i as u32);
            }
            res
        })
        .sum::<u64>();

    println!("{p2:?}");

    Ok(())
}
