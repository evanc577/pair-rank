use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::{
    fmt,
    fs::File,
    io::{prelude::*, stdin, stdout, BufReader},
    path::Path,
};

enum Choice {
    Left,
    Right,
}

fn main() {
    // read input data
    let candidates = lines_from_file("input.txt");
    println!("{:?}", candidates);
    let candidate_refs: Vec<&str> = candidates.iter().map(|s| s.as_str()).collect();

    // sort data
    let order = kwiksort(candidate_refs);
    println!();
    println!("Final ordering:");
    for (i, s) in order.iter().enumerate() {
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

fn kwiksort<T: fmt::Display + ?Sized>(list: Vec<&T>) -> Vec<&T> {
    if list.len() <= 1 {
        return list;
    }

    let mut rng = thread_rng();

    // Choose random pivot
    let i: usize = rng.gen_range(0..list.len());
    let mut list_l = vec![];
    let mut list_r = vec![];

    // iterate through all elements except pivot
    let mut iter = list[0..i]
        .iter()
        .chain(list[i + 1..list.len()].iter())
        .collect::<Vec<_>>();

    // pairwise comparison of pivot with every other element
    iter.shuffle(&mut rng);
    for e in iter {
        match compare(list[i], e) {
            Choice::Left => list_l.push(*e),
            Choice::Right => list_r.push(*e),
        }
    }

    // concatenate
    let mut ret = kwiksort(list_l);
    ret.push(list[i]);
    ret.extend(kwiksort(list_r));
    ret
}

fn compare<T: fmt::Display + ?Sized>(choice_1: &T, choice_2: &T) -> Choice {
    println!();
    println!("Pick:");
    println!("(1) {}", choice_1);
    println!("(2) {}", choice_2);
    print!("Your choice: ");
    stdout().flush().unwrap();

    let mut buffer = String::new();
    stdin().lock().read_line(&mut buffer).unwrap();
    let buffer = buffer.trim();
    if buffer == "1" {
        return Choice::Right;
    } else if buffer == "2" {
        return Choice::Left;
    }
    println!("{:?}", buffer);
    panic!("invalid input!");
}
