use std::io::BufRead;
use std::{fs::File, io::BufReader};

use itertools::Itertools;

fn main() -> std::io::Result<()> {
    let input_file = "myinput";

    let tree = BufReader::new(File::open(input_file)?)
        .lines()
        .map_while(|l| l.ok())
        .map(|l| {
            let mut split = l.split(":");

            let enter = String::from(split.next().unwrap());
            let exits = split
                .next()
                .unwrap()
                .split(" ")
                .skip(1)
                .map(String::from)
                .collect_vec();

            (enter, exits)
        })
        .collect_vec();

    let p1 = count_exits_no_cycle(&tree, "you");
    println!("p1: {:?}", p1);

    Ok(())
}

pub fn count_exits_no_cycle(tree: &[(String, Vec<String>)], current: &str) -> i32 {
    let next = tree.iter().find(|c| c.0 == current).unwrap();

    next.1.iter().map(|out| {
        if out == "out" {
            1
        } else {
            count_exits_no_cycle(tree, out)
        }
    }).sum()
}
