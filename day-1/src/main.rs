use std::io::BufRead;
use std::collections::HashMap;

use std::{fs::File, io::BufReader};

fn main() -> std::io::Result<()> {
    let _ = BufReader::new(File::open("input")?)
        .lines();

    Ok(())
}
