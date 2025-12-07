use itertools::Itertools;
use std::collections::HashMap;
use std::io::BufRead;

use std::{fs::File, io::BufReader};
fn main() -> std::io::Result<()> {
    let input_file = "myinput";
    let p1 = BufReader::new(File::open(input_file)?)
        .lines()
        .map_while(|l| l.ok())
        .map(|l| l.chars().collect_vec())
        .fold(
            (0_i32, vec![] as Vec<char>),
            |(counter, beams), current_line| {
                // Handle start
                if let Some((pos, _)) = current_line.iter().enumerate().find(|c| *c.1 == 'S') {
                    let mut beams = current_line.clone();
                    beams[pos] = '|';
                    return (0, beams);
                }

                // Can we do it without this clone
                let mut new_beams = beams.clone();

                let total = current_line
                    .clone()
                    .into_iter()
                    .enumerate()
                    .filter(|c| c.1 == '^')
                    .filter(|(pos, _)| beams[*pos] == '|')
                    .map(|(pos, _)| pos)
                    .map(|pos| {
                        new_beams[pos - 1] = '|';
                        new_beams[pos] = '.';
                        new_beams[pos + 1] = '|';
                    })
                    .count() as i32;

                (counter + total, new_beams)
            },
        );

    println!("p1: {:?}", p1.0);

    let manifold = BufReader::new(File::open(input_file)?)
        .lines()
        .map_while(|l| l.ok())
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let start = manifold[0].iter().position(|c| *c == 'S').unwrap();
    let p2 = recurse_timeline(&manifold, 0, start, &mut HashMap::new());

    println!("p2: {:?}", p2);

    Ok(())
}

pub fn recurse_timeline(
    manifold: &Vec<Vec<char>>,
    step: usize,
    beam_pos: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if memo.contains_key(&(step, beam_pos)) {
        return memo[&(step, beam_pos)];
    }

    if manifold.len() == step {
        return 1;
    }

    let res = if manifold[step][beam_pos] == '^' {
        recurse_timeline(manifold, step + 1, beam_pos - 1, memo)
            + recurse_timeline(manifold, step + 1, beam_pos + 1, memo)
    } else {
        recurse_timeline(manifold, step + 1, beam_pos, memo)
    };

    memo.insert((step, beam_pos), res);
    res
}
