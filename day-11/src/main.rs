use std::collections::HashMap;
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

    let p2 = count_exits_p2(&tree, &[false, false], "svr", &mut HashMap::new());
    println!("p2: {:?}", p2);

    Ok(())
}

pub fn count_exits_no_cycle(tree: &[(String, Vec<String>)], current: &str) -> i32 {
    let next = tree.iter().find(|c| c.0 == current).unwrap();

    next.1
        .iter()
        .map(|out| {
            if out == "out" {
                1
            } else {
                count_exits_no_cycle(tree, out)
            }
        })
        .sum()
}

pub fn count_exits_p2(
    tree: &[(String, Vec<String>)],
    requirements: &[bool; 2],
    current: &str,
    memo: &mut HashMap<(String, [bool; 2]), i64>,
) -> i64 {
    if memo.contains_key(&(current.to_string(), *requirements)) {
        return *memo.get(&(current.to_string(), *requirements)).unwrap();
    }

    let next = tree.iter().find(|c| c.0 == current).unwrap();

    let res = next
        .1
        .iter()
        .map(|out| {
            let req = if out == "dac" {
                [true, requirements[1]]
            } else if out == "fft" {
                [requirements[0], true]
            } else {
                *requirements
            };

            if out == "out" {
                if requirements[0] && requirements[1] {
                    return 1;
                }
                0
            } else {
                count_exits_p2(tree, &req, out, memo)
            }
        })
        .sum();

    memo.insert((current.to_string(), *requirements), res);
    res
}
