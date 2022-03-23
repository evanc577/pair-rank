mod element;
mod merge_insertion_sort;
mod merge_sort;

use clap::{Arg, App, arg_enum, value_t};
use self::element::Element;
use self::merge_insertion_sort::merge_insertion_sort;
use self::merge_sort::merge_sort;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

arg_enum!{
    #[derive(PartialEq, Debug)]
    pub enum SortAlgorithm {
        Stdlib,
        MergeInsertion,
        Merge,
    }
}

fn main() {
    let args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("SORT")
                .short("s")
                .long("sort")
                .value_name("algorithm")
                .possible_values(&SortAlgorithm::variants())
                .case_insensitive(true)
                .default_value("MergeInsertion"),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    // read input data
    let candidates: Vec<_> = lines_from_file(args.value_of("INPUT").unwrap()).collect();

    // sort/rank data
    let mut ranking: Vec<_> = candidates
        .iter()
        .map(|s| Element::new(s.as_str()))
        .collect();
    match value_t!(args, "SORT", SortAlgorithm).unwrap() {
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
