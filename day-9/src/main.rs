use itertools::Itertools;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

use std::cmp;

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

    let shape = BufReader::new(File::open(input_file)?)
        .lines()
        .map_while(|l| l.ok())
        .map(|l: String| {
            let (a, b) = l.split(",").collect_tuple().unwrap();
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .collect_vec();

    let lines = BufReader::new(File::open(input_file)?)
        .lines()
        .map_while(|l| l.ok())
        .map(|l: String| {
            let (a, b) = l.split(",").collect_tuple().unwrap();
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .collect_vec();

    // too low:  93142
    // too low:  700713
    // not good  250435892
    // too high: 4767418746
    let p2 = lines
        .clone()
        .iter()
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

            lines
                .iter()
                .tuple_windows()
                .filter(|(p1, p2)| {
                    if p1.1 == p2.1 {
                        // outside
                        if !((min_y > p1.1 && min_y > p2.1) || (max_y < p1.1 && max_y < p2.1)) {
                            let p_min_x = p1.0.min(p2.0);
                            let p_max_x = p1.0.max(p2.0);
                            return !(p_max_x < min_x || p_min_x > max_x)
                        }
                        false
                    } else if p1.0 == p2.0 {
                        if !((min_x > p1.0 && min_x > p2.0) || (max_x < p1.0 && max_x < p2.0)) {
                            let p_min_y = p1.1.min(p2.1);
                            let p_max_y = p1.1.max(p2.1);

                            return !(p_max_y < min_y || p_min_y > max_y)
                        }
                        false
                    } else {
                        unreachable!()
                    }
                })
                .count()
                == 0
        })
        .map(|(a, _, _, b)| {
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

    println!("p2: {:?}", p2);

    Ok(())
}
