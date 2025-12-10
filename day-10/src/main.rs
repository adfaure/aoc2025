use std::collections::VecDeque;
use std::collections::HashSet;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

use itertools::Itertools;

fn parse_line(s: String) -> (Vec<char>, Vec<Vec<i32>>, Vec<i32>) {
    let mut line = s.split(" ").map(String::from).collect_vec();
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
}

fn main() -> std::io::Result<()> {
    let input_file = "myinput";

    let p1 = BufReader::new(File::open(input_file)?)
        .lines()
        .map_while(|l| l.ok())
        .map(parse_line)
        .map(|(indicators, buttons, _)| solve_p1(&indicators, &buttons))
        .sum::<i32>();

    println!("p1: {:?}", p1);

    Ok(())
}

fn solve_p1(desired_state: &[char], buttons: &[Vec<i32>]) -> i32 {
    let start = vec!['.'; desired_state.len()];
    let mut seen = HashSet::new();

    let mut deq = VecDeque::from(
        (0..buttons.len())
            .map(|button| (0, button, start.clone()))
            .collect_vec(),
    );

    while let Some((idx, button_idx, indicators)) = deq.pop_front() {
        if idx == 100 {
            // Some bound so that I don't crash my laptop again
            panic!()
        }

        if seen.insert((button_idx, indicators.clone())) {
            if indicators == desired_state {
                return idx
            }

            let mut pushed = indicators.clone();

            buttons[button_idx].iter().for_each(|idx| {
                if pushed[*idx as usize] == '.' {
                    pushed[*idx as usize] = '#';
                } else {
                    pushed[*idx as usize] = '.';
                }
            });

            for button in 0..buttons.len() {
                deq.push_back((idx + 1, button, pushed.clone()));
            }
        }
    }

    unreachable!()
}
