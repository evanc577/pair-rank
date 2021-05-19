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
    let candidates = lines_from_file("input.txt");

    // sort data
    let order = candidates.iter().map(|s| Element::new(s.as_str())).sorted();

    // display ordering
    println!();
    println!("Final ordering:");
    for (i, s) in order.enumerate() {
        println!("{}. {}", i + 1, s);
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
