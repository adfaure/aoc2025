use std::io::BufRead;
use std::{fs::File, io::BufReader};

use itertools::Itertools;

fn main() -> std::io::Result<()> {
    let input_file = "input";

    let p1 = BufReader::new(File::open(input_file)?)
        .lines()
        .map_while(|l| l.ok())
        .map(|l: String| {
            let mut line = l.split(" ").map(String::from).collect_vec();
            let indicators = line[0]
                .clone()
                .chars()
                .skip(1)
                .take_while(|c| *c != ']')
                .collect_vec();

            let p2 = line
                .pop()
                .unwrap()
                .replace("{", "")
                .replace("}", "")
                .split(",")
                .map(|c| c.parse::<i32>().unwrap())
                .collect_vec();

            let buttons = line
                .into_iter()
                .skip(1)
                .map(|button| {
                    button
                        .replace("(", "")
                        .replace(")", "")
                        .split(",")
                        .map(|c| c.parse::<i32>().unwrap())
                        .collect_vec()
                })
                .collect_vec();

            (indicators, buttons, p2)
        })
        .collect_vec();

    println!("p1: {:?}", p1);

    Ok(())
}
