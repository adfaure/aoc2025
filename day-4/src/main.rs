use itertools::Itertools;
use std::io::BufRead;

use std::{fs::File, io::BufReader};
fn main() -> std::io::Result<()> {
    let grid = BufReader::new(File::open("myinput")?)
        .lines()
        .map_while(|l| l.ok())
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    let (h, w) = (grid.len() as i32, grid.get(0).unwrap().len() as i32);

    let p1 = (0..h)
        .cartesian_product(0..w)
        .filter(|(y, x)| grid.get(*y as usize).unwrap().get(*x as usize).unwrap() == &'@')
        .map(|(ref y, ref x): (i32, i32)| {
            (-1isize..=1)
                .cartesian_product(-1isize..=1)
                .map(|(ref ny, ref nx)| (*y + *ny as i32, *x + *nx as i32))
                .filter(|(ny, nx)| {
                    *ny < h && *ny >= 0 && *nx < w && *nx >= 0 && !(*nx == *x && *ny == *y)
                })
                .filter(|(ny, nx)| {
                    grid.get(*ny as usize).unwrap().get(*nx as usize).unwrap() == &'@'
                })
                .count() as u64
        })
        .filter(|c| *c < 4)
        .count();

    println!("p1: {p1:?}");

    Ok(())
}
