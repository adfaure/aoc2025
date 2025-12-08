use itertools::Itertools;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::iter::once;
use std::rc::Rc;
use std::{fs::File, io::BufReader};

fn dist3d(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> i64 {
    ((a.0 - b.0).abs().pow(2) + (a.1 - b.1).abs().pow(2) + (a.2 - b.2).abs().pow(2)).isqrt()
}

type Classes = Vec<HashSet<(i64, i64, i64)>>;

fn main() -> std::io::Result<()> {
    let input_file = "input";
    let junction_boxes = BufReader::new(File::open(input_file)?)
        .lines()
        .map_while(|l| l.ok())
        .flat_map(|l| {
            l.split(",")
                .map(|str| str.parse::<i64>().unwrap())
                .collect_tuple::<(_, _, _)>()
        })
        .collect_vec();

    let total = 10;

    let mut classes: Classes = junction_boxes
        .clone()
        .into_iter()
        .map(|a| HashSet::from([a]))
        .collect();
    for _ in 0..total {
        let min_dist = classes
            .iter()
            .enumerate()
            .permutations(2)
            .min_by(|cl1, cl2| {
                let a = cl1[0]
                    .1
                    .iter()
                    .cartesian_product(cl1[1].1.iter())
                    .map(|e| (e, dist3d(e.0, e.1)))
                    .min_by(|a, b| a.1.cmp(&b.1))
                    .unwrap();

                let b = cl2[0]
                    .1
                    .iter()
                    .cartesian_product(cl2[1].1.iter())
                    .map(|e| (e, dist3d(e.0, e.1)))
                    .min_by(|a, b| a.1.cmp(&b.1))
                    .unwrap();
                a.1.cmp(&b.1)
            })
            .unwrap();

        let left: HashSet<_> = min_dist[0].1.clone();
        let mut new_class: HashSet<_> = min_dist[1].1.clone();
        new_class.extend(&mut left.iter());

        classes = classes
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(idx, _)| *idx != min_dist[0].0 && *idx != min_dist[1].0)
            .map(|(_, item)| item)
            .chain(once(new_class))
            .collect_vec();

        for c in &classes {
            println!("{:?} - len {}", c, c.len());
        }

        println!("------");

    }

    let p1 = classes
        .iter()
        .map(|class| class.len())
        .sorted()
        .rev()
        .take(3)
        .product::<usize>();

    println!("p1: {}", p1);

    Ok(())
}
