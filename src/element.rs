use std::{
    cmp::Ordering,
    fmt,
    io::{prelude::*, stdin, stdout},
};

#[derive(PartialEq, Eq)]
pub struct Element<T>
where
    T: fmt::Display + Eq,
{
    name: T,
}

impl<T> Element<T>
where
    T: fmt::Display + Eq,
{
    pub fn new(name: T) -> Element<T> {
        Element { name }
    }
}

impl<T> fmt::Display for Element<T>
where
    T: fmt::Display + Eq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<T> PartialOrd for Element<T>
where
    T: fmt::Display + Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        loop {
            println!();
            println!("(1) {}", self.name);
            println!("(2) {}", other.name);
            print!("Your choice: ");
            stdout().flush().unwrap();

            let mut buffer = String::new();
            stdin().lock().read_line(&mut buffer).unwrap();
            match buffer.trim() {
                "1" => return Some(Ordering::Less),
                "2" => return Some(Ordering::Greater),
                _ => {
                    println!("\nInvalid number!");
                },
            };
        }
    }
}

impl<T> Ord for Element<T>
where
    T: fmt::Display + Eq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}
