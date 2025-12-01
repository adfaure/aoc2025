use std::io::BufRead;

use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    let mut cur = 50;
    let r = BufReader::new(File::open("input")?)
        .lines()
        .map_while(|l| l.ok())
        .map(|mut l| {
            if l.starts_with('L') {
                -l.split_off(1).parse::<i32>().unwrap()
            } else {
                l.split_off(1).parse::<i32>().unwrap()
            }
        })
        .filter(|n| {
            cur = ((cur + 100) + n) % 100;
            cur == 0
        })
        .count();
    println!("p1: {}", r);

    let mut cur = 50;

    // 5747 too low
    // 5799 too low
    // 5882 ?
    // 6268 too high
    // 6693 nop
    let r = BufReader::new(File::open("input")?)
        .lines()
        .map_while(|l| l.ok())
        .map(|mut l| {
            if l.starts_with('L') {
                -l.split_off(1).parse::<i32>().unwrap()
            } else {
                l.split_off(1).parse::<i32>().unwrap()
            }
        })
        .map(|n| {
            let mut res = 0;

            // compute overflow
            let of = n.abs() / 100;

            // bound n
            let n = if n < 0 {
                n % -100
            } else {
                n % 100
            };

            // Compute the next value
            let new_t = ((cur + 100) + n) % 100;


            // Either next will be zero or we cross the 0 zero in one direction
            if new_t == 0 || cur != 0 && (n < 0 && n.abs() > cur || n > 0 && n > (100 - cur)) {
                res = 1;
            }

            cur = new_t;
            res + of
        })
        .sum::<i32>();
    println!("{}", r);

    Ok(())
}
