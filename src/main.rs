mod element;

use self::element::Element;
use itertools::Itertools;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    // read input data
    let input_file = std::env::args_os().nth(1).expect("expected 1 argument");
    let candidates: Vec<_> = lines_from_file(input_file).collect();

    // sort/rank data
    let ranking = candidates.iter().map(|s| Element::new(s.as_str())).sorted();

    // display ordering
    println!("\nFinal ordering:");
    for (i, s) in ranking.enumerate() {
        println!("{}. {}", i + 1, s);
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> impl Iterator<Item = String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line"))
}
