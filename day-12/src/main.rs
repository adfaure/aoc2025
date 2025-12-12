use std::io::BufRead;
use std::{fs::File, io::BufReader};

use itertools::Itertools;

fn main() -> std::io::Result<()> {
    let input_file = "myinput";

    let shapes: Vec<_> = BufReader::new(File::open(input_file)?)
        .lines()
        .map_while(|l| l.ok())
        .take(30)
        .chunks(5)
        .into_iter()
        .map(|v| {
            let (_, l1, l2, l3, _) = v.collect_tuple().unwrap();

            let shape = vec![
                l1.chars().collect_vec(),
                l2.chars().collect_vec(),
                l3.chars().collect_vec(),
            ];
            (
                shape.clone(),
                shape
                    .iter()
                    .flat_map(|l| l.iter().filter(|c| **c == '#'))
                    .count(),
            )
        })
        .collect_vec();

    let constraints: Vec<_> = BufReader::new(File::open(input_file)?)
        .lines()
        .map_while(|l| l.ok())
        .skip(30)
        .map(|l| {
            println!("{l:?}");

            let (w, h) = l
                .split(":")
                .next()
                .unwrap()
                .split("x")
                .map(|size| size.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();

            let constraints = l
                .split(":")
                .last()
                .unwrap()
                .split(" ")
                .skip(1)
                .map(|nb| nb.parse::<usize>().unwrap())
                .collect_vec();
            (w, h, constraints)
        })
        .collect_vec();

    // Lets check if the size total size is enough
    // constraints.iter().map(|(w, h, constraints)| {
    //     shape
    // })

    let test = constraints
        .iter()
        .filter(|(w, h, counts)| {
            counts
                .iter()
                .enumerate()
                .map(|(idx, count)| count * shapes[idx].1)
                .sum::<usize>() <= w*h
        })
        .count();

    println!("test: {} / {} are possible", test, constraints.len());

    // println!("p1: {:?}", constraints);

    Ok(())
}
