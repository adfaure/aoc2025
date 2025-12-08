use itertools::Itertools;
use std::collections::HashSet;
use std::io::BufRead;
use std::iter::once;
use std::{fs::File, io::BufReader};

fn dist3d(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> i64 {
    // No sqrt to gain time
    (a.0 - b.0).abs().pow(2) + (a.1 - b.1).abs().pow(2) + (a.2 - b.2).abs().pow(2)
}

type Classes = Vec<HashSet<(i64, i64, i64)>>;

fn main() -> std::io::Result<()> {
    let input_file = "myinput";

    let junction_boxes = BufReader::new(File::open(input_file)?)
        .lines()
        .map_while(|l| l.ok())
        .flat_map(|l| {
            l.split(",")
                .map(|str| str.parse::<i64>().unwrap())
                .collect_tuple::<(_, _, _)>()
        })
        .collect_vec();

    // Create a vector containing all the start classes
    let mut classes: Classes = junction_boxes
        .clone()
        .into_iter()
        .map(|a| HashSet::from([a]))
        .collect();

    // create all permutation
    for (idx, min_dist) in junction_boxes
        .clone()
        .into_iter()
        .combinations(2)
        .map(|cl1| (cl1.clone(), dist3d(&cl1[0], &cl1[1])))
        .sorted_by(|a, b| a.1.cmp(&b.1))
        .map(|(c, _)| c)
        .enumerate()
    {

        // Find the two classes associated to the
        // boxes and merge them
        let left = classes
            .iter()
            .enumerate()
            .find(|(_, class)| class.contains(&min_dist[0]))
            .unwrap();

        let mut new_class = classes
            .clone()
            .into_iter()
            .enumerate()
            .find(|(_, class)| class.contains(&min_dist[1]))
            .unwrap();

        new_class.1.extend(&mut left.1.iter());

        classes = classes
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(idx, _)| *idx != left.0 && *idx != new_class.0)
            .map(|(_, item)| item)
            .chain(once(new_class.1.clone()))
            .collect_vec();

        // go up to ten for example
        if idx - 1 == 10 {
            let p1 = classes
                .iter()
                .map(|class| class.len())
                .sorted()
                .rev()
                .take(3)
                .product::<usize>();

            println!("example input: {}", p1);
        }

        if idx - 1 == 1_000 {
            let p1 = classes
                .iter()
                .map(|class| class.len())
                .sorted()
                .rev()
                .take(3)
                .product::<usize>();

            println!("p1: {}", p1);
        }

        if classes.len() == 1 {
            println!("p2: {:?}", min_dist[0].0 * min_dist[1].0);
            break;
        }
    }

    Ok(())
}
