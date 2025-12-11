use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::BufRead;
use std::{fs::File, io::BufReader};

use itertools::Itertools;

extern crate lp_modeler;

use lp_modeler::dsl::*;
use lp_modeler::solvers::{CbcSolver, SolverTrait};

fn parse_line(s: String) -> (Vec<char>, Vec<Vec<i32>>, Vec<usize>) {
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
        .map(|c| c.parse::<usize>().unwrap())
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

    let p2 = BufReader::new(File::open(input_file)?)
        .lines()
        .map_while(|l| l.ok())
        .map(parse_line)
        .map(|(_, buttons, jolts)| solve_p2(&jolts, &buttons))
        .sum::<i32>();

    println!("p2: {:?}", p2);
    Ok(())
}

fn solve_p2(desired_state: &[usize], buttons: &[Vec<i32>]) -> i32 {

    let vars = buttons
        .iter()
        .enumerate()
        .map(|(idx, button)| (button, LpInteger::new(&format!("k{idx}"))))
        .collect_vec();

    let mut problem = LpProblem::new("p2", LpObjective::Minimize);

    // Define Objective Function
    let obj_vec: Vec<LpExpression> = (vars.iter().map(|(_, var)| {
            var * 1
       })).collect();

    problem += obj_vec.sum();

    for (i, state) in desired_state.iter().enumerate() {
        let state_vars = vars.iter().filter(|c| c.0.contains(&(i as i32))).collect_vec();
        problem += sum(&state_vars, |&var| var.1.clone()).equal(*state as i32)
    }

    // Specify solver
    let solver = CbcSolver::new();

    // Run optimisation and process output hashmap
    match solver.run(&problem) {
        Ok(solution) => {
            solution.results.values().map(|v| *v as i32 ).sum::<i32>()
        },
        Err(msg) => unreachable!("{msg:?}. Did you forget to install cbc solver ? `nix shell nixpkgs#cbc` ;)"),
    }
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
                return idx;
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
