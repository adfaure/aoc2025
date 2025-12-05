use std::io::Read;

use itertools::Itertools;
use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    let mut input = Default::default();
    let _ = BufReader::new(File::open("input")?).read_to_string(&mut input);

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

    let p2 = input
        .trim()
        .split(',')
        .map(|e| {
            e.split('-')
                .map(|e| e.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .flat_map(|(min, max)| {
            (min..=max).filter_map(|n: u64| {
                let mut nb_digit = n.ilog10() + 1;
                if n == 0 {
                    nb_digit = 1;
                }

                // println!("<<<<--- {n} --->>>>");
                // 111 111 - 11 11 11  - 1 1 1 1 1 1
                // 2         3           1
                (1..nb_digit)
                    .filter(|nd_i| nb_digit.is_multiple_of(*nd_i))
                    .find_map(|n_split| {
                        let mut sub_n = n;
                        let n_times = nb_digit / n_split;

                        if (0..n_times).map(|_| {
                            let r = sub_n % 10_i32.pow(n_split) as u64;
                            sub_n /= 10_i32.pow(n_split) as u64;
                            // print!("{r}, ");
                            r
                        }).all_equal() {
                            return Some(n);
                        }

                        None
                    })
            })
        })
        .sum::<u64>();

    println!("p2: {}", p2);
    Ok(())
}
