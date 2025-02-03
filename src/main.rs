mod element;
mod merge_insertion_sort;
mod merge_sort;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use clap::Parser;
use clap::ValueEnum;
use rand::seq::SliceRandom;

use self::element::Element;
use self::merge_insertion_sort::merge_insertion_sort;
use self::merge_sort::merge_sort;

#[derive(Parser)]
struct Cli {
    inputs_file: PathBuf,

    #[arg(short, long, value_enum, default_value_t = SortAlgorithm::Merge)]
    sort_algorithm: SortAlgorithm,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SortAlgorithm {
    Stdlib,
    MergeInsertion,
    Merge,
}

fn main() {
    let args = Cli::parse();

    // read input data
    let mut candidates: Vec<_> = lines_from_file(args.inputs_file).collect();
    let mut rng = rand::rng();
    candidates.shuffle(&mut rng);

    // sort/rank data
    let mut ranking: Vec<_> = candidates
        .iter()
        .map(|s| Element::new(s.as_str()))
        .collect();
    match args.sort_algorithm {
        SortAlgorithm::Stdlib => ranking.sort(),
        SortAlgorithm::Merge => merge_sort(&mut ranking[..]),
        SortAlgorithm::MergeInsertion => merge_insertion_sort(&mut ranking[..]),
    };

    // display ordering
    println!("\nFinal ordering:");
    for (i, s) in ranking.iter().enumerate() {
        println!("{}. {}", i + 1, s);
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> impl Iterator<Item = String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines().map(|l| l.expect("Could not parse line"))
}
